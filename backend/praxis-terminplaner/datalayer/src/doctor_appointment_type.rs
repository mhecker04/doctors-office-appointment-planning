use std::future::IntoFuture;

use async_trait::async_trait;
use entities::doctor_appointment_type;
use models::doctor_appointment_type::DoctorAppointmentTypeModel;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};

use crate::{
    base::{map_to_model, map_to_vector, ListRepository, Repository},
    error::RepositoryError,
    implement_repository,
    sea::{map_sea_orm_error, SeaOrmRepository},
};

pub struct DoctorAppointmentTypeRepository;

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        entities::doctor_appointment_type::Entity,
        entities::doctor_appointment_type::ActiveModel,
        DoctorAppointmentTypeModel,
        String,
    > for DoctorAppointmentTypeRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

#[async_trait]
impl ListRepository<DoctorAppointmentTypeModel, String> for DoctorAppointmentTypeRepository {

    async fn get_by_parent_id(
        &self,
        parent_id: &String,
    ) -> Result<Vec<DoctorAppointmentTypeModel>, RepositoryError> {
        let connection = self.get_connection().await;

        let sea_models = entities::doctor_appointment_type::Entity::find()
            .filter(doctor_appointment_type::Column::DoctorId.eq(parent_id))
            .all(&connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;


        let related_appointment_types = sea_models
            .load_one(entities::appointment_type::Entity, &connection)
            .await
            .map_err(map_sea_orm_error)?;

        let mut doctor_appointment_type_models = Vec::new();

        for iterator in sea_models.into_iter().zip(related_appointment_types) {
            let (doctor_appointment_type_sea_model, appointment_type_sea_model) = iterator;

            match appointment_type_sea_model {
                Some(appointment_type_sea_model) => {
                    let mut doctor_appointment_type_model: DoctorAppointmentTypeModel =
                        map_to_model(&doctor_appointment_type_sea_model)?;
                    let appointment_type_model = map_to_model(&appointment_type_sea_model)?;
                    doctor_appointment_type_model.appointment_type = Some(appointment_type_model);
                    doctor_appointment_type_models.push(doctor_appointment_type_model);
                }
                None => {}
            }   
        }


        Ok(doctor_appointment_type_models)
    }
}

impl DoctorAppointmentTypeRepository {
    pub async fn get_by_appointment_type_id(
        &self,
        appointment_type_id: &String,
    ) -> Result<Vec<DoctorAppointmentTypeModel>, RepositoryError> {

        let connection = self.get_connection().await;

        let doctor_appointment_type_sea_models = entities::doctor_appointment_type::Entity::find()
            .filter(
                entities::doctor_appointment_type::Column::AppointmentTypeId
                    .eq(appointment_type_id),
            )
            .all(&connection)
            .await
            .map_err(map_sea_orm_error)?;

        map_to_vector(&doctor_appointment_type_sea_models)
    }
}

implement_repository!(
    DoctorAppointmentTypeRepository,
    DoctorAppointmentTypeModel,
    String
);
