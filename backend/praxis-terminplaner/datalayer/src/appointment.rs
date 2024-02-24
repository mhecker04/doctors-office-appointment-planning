use async_trait::async_trait;
use chrono::{NaiveDateTime, Timelike, Duration, NaiveTime};
use futures::future::err;
use models::{
    appointment::AppointmentModel, appointment_type::AppointmentTypeModel,
    available_appointment_resources::{self, AvailableAppointmentRessourcesModel},
};
use sea_orm::{
    sea_query::Query, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter
};

use crate::{base::{map_to_vector, Repository}, error::RepositoryError, sea::SeaOrmRepository, implement_repository};

pub struct AppointmentRepository;

fn add_naive_time_to_naive_date_time(naive_date_time: &NaiveDateTime, naive_time: &NaiveTime) -> NaiveDateTime {

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
                                Condition::any()
                                    .add(entities::appointment::Column::From.gt(to))
                                    .add(
                                        entities::appointment::Column::To.lt(datetime)
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
                                Condition::any()
                                        .add(entities::appointment::Column::From.gt(to))
                                        .add(
                                        entities::appointment::Column::To.lt(datetime)
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



        Ok(AvailableAppointmentRessourcesModel {
            doctors: map_to_vector(&available_doctor_entities)?,
            rooms: map_to_vector(&available_room_entities)?
        })

    }
}
