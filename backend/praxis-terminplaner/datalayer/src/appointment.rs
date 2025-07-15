use async_trait::async_trait;
use chrono::{NaiveDateTime, Timelike, Duration, NaiveTime};
use models::{
    appointment::AppointmentModel, appointment_type::AppointmentTypeModel,
    available_appointment_resources::AvailableAppointmentRessourcesModel, doctor::DoctorModel,
};
use sea_orm::{
    sea_query::{Query, SimpleExpr}, ColumnTrait, Condition, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter
};

use crate::{base::{map_to_model, map_to_vector, Repository}, error::RepositoryError, implement_repository, sea::{map_sea_orm_error, SeaOrmRepository}};

pub struct AppointmentRepository;

pub fn add_naive_time_to_naive_date_time(naive_date_time: &NaiveDateTime, naive_time: &NaiveTime) -> NaiveDateTime {

    *naive_date_time + Duration::hours(i64::from(naive_time.hour()))
        + Duration::minutes(i64::from(naive_time.minute()))
        + Duration::seconds(i64::from(naive_time.second()))

}

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        entities::appointment::Entity,
        entities::appointment::ActiveModel,
        AppointmentModel,
        String,
    > for AppointmentRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

implement_repository!(AppointmentRepository, AppointmentModel, String);

impl AppointmentRepository {

    pub async fn get_conflicting_appointments(&self, appointment_type_id: &String) -> Result<Vec<AppointmentModel>, RepositoryError> {
        let connection = self.get_connection().await;

        let conflicting_appointment_entities_result = entities::appointment::Entity::find()
            .filter(
                Condition::all()
                    .add(self.get_doctor_has_appointment_type_predicate(appointment_type_id))
                    .add(self.get_room_has_appointment_type_predicate(appointment_type_id)))
            .all(&connection)
            .await;

        match conflicting_appointment_entities_result {
            Ok(conflicting_appointment_entities) => {
                map_to_vector(&conflicting_appointment_entities)
            },
            Err(err) => {
                println!("{}", err);
                Err(map_sea_orm_error(err))
            }
        }

    }

    pub async fn get_available_ressources(
        &self,
        datetime: NaiveDateTime,
        appointment_type: &AppointmentTypeModel,
    ) -> Result<AvailableAppointmentRessourcesModel, RepositoryError> {

        let to = add_naive_time_to_naive_date_time(&datetime, &appointment_type.duration);

        let connection = self.get_connection().await;

        let appointment_type_id = appointment_type.appointment_type_id.clone().unwrap();

        let available_doctor_entities = entities::doctor::Entity::find()
            .filter(
                Condition::all()
                .add(
                    entities::doctor::Column::DoctorId.not_in_subquery(
                            Query::select()
                                    .column(entities::appointment::Column::DoctorId)
                                    .from(entities::appointment::Entity)
                                    .cond_where(
                                Condition::all()
                                    .add(entities::appointment::Column::StartDateTime.lt(to))
                                    .add(
                                        entities::appointment::Column::EndDateTime.gt(datetime)
                                    )
                                    ).to_owned()
                    ),
                ).add(
                    entities::doctor::Column::DoctorId.in_subquery(
                        Query::select()
                            .column(entities::doctor_appointment_type::Column::DoctorId)                        
                            .from(entities::doctor_appointment_type::Entity)
                            .cond_where(
                                Condition::all()
                                .add(entities::doctor_appointment_type::Column::AppointmentTypeId.eq(appointment_type_id.clone()))
                            ).to_owned()
                    )
                ))
            .all(&connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        let available_room_entities = entities::room::Entity::find()
            .filter(
                Condition::all()
                .add(
                    entities::room::Column::RoomId.not_in_subquery(
                            Query::select()
                                    .column(entities::appointment::Column::RoomId)
                                    .from(entities::appointment::Entity)
                                    .cond_where(
                                Condition::all()
                                        .add(entities::appointment::Column::StartDateTime.lt(to))
                                        .add(
                                        entities::appointment::Column::EndDateTime.gt(datetime)
                                        )
                                    ).to_owned()

                    ),
                ).add(
                    entities::room::Column::RoomId.in_subquery(
                        Query::select()
                            .column(entities::room_appointment_type::Column::RoomId)                        
                            .from(entities::room_appointment_type::Entity)
                            .cond_where(
                                Condition::all()
                                .add(entities::room_appointment_type::Column::AppointmentTypeId.eq(appointment_type_id))
                            ).to_owned()
                    )

                ))
            .all(&connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        let mut doctors = Vec::new();

        let related_person_entities = available_doctor_entities.load_one(entities::person::Entity, &self.get_connection().await)
            .await
            .map_err(map_sea_orm_error)?;

        for iterator in available_doctor_entities.into_iter().zip(related_person_entities) {
            let (doctor_sea_model, person_sea_model) = iterator;

            let mut doctor_model: DoctorModel = map_to_model(&doctor_sea_model)?;
            match(person_sea_model) {
                Some(person_sea_model) => {
                    let person_model = map_to_model(&person_sea_model)?;
                    doctor_model.person = Some(person_model);
                },
                None => {

                }
            }
            doctors.push(doctor_model)
        }

        Ok(AvailableAppointmentRessourcesModel {
            doctors,
            rooms: map_to_vector(&available_room_entities)?
        })

    }

    fn get_doctor_has_appointment_type_predicate(&self, appointment_type_id: &String) -> SimpleExpr {
        entities::appointment::Column::DoctorId.in_subquery(
            Query::select()
            .column(entities::doctor_appointment_type::Column::DoctorId)                        
            .from(entities::doctor_appointment_type::Entity)
            .cond_where(
                Condition::all()
                .add(entities::doctor_appointment_type::Column::AppointmentTypeId.eq(appointment_type_id))
            ).to_owned()
        )
    }

    fn get_room_has_appointment_type_predicate(&self, appointment_type_id: &String) -> SimpleExpr {
        entities::appointment::Column::RoomId.in_subquery(
            Query::select()
            .column(entities::room_appointment_type::Column::RoomId)                        
            .from(entities::room_appointment_type::Entity)
            .cond_where(
                Condition::all()
                .add(entities::room_appointment_type::Column::AppointmentTypeId.eq(appointment_type_id))
            ).to_owned()
        )
    }

}
