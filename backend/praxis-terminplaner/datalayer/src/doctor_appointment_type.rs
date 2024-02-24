use std::result;

use async_trait::async_trait;
use entities::{doctor_appointment_type, appointment_type, doctor};
use models::{doctor_appointment_type::DoctorAppointmentTypeModel, appointment_type::AppointmentTypeModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, LoaderTrait};

use crate::{
    base::{ListRepository, Repository, map_to_vector, map_to_model},
    error::RepositoryError,
    implement_repository,
    sea::SeaOrmRepository,
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

        let entities = entities::doctor_appointment_type::Entity::find()
            .filter(doctor_appointment_type::Column::DoctorId.eq(parent_id))
            .all(&connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        let related_appointment_types = entities.load_one(appointment_type::Entity, &connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        let doctor_appointment_types: Vec<DoctorAppointmentTypeModel> = map_to_vector(&entities)?;

        let mut result_models: Vec<DoctorAppointmentTypeModel> = Vec::new();

        let mut i = 0;

        for mut doctor_appointment_type in doctor_appointment_types {
            let appointment_type_model: AppointmentTypeModel = map_to_model(related_appointment_types.get(i).unwrap())?;

            doctor_appointment_type.appointment_type = Some(appointment_type_model);

            i += 1;
            result_models.push(doctor_appointment_type)

        }

        Ok(result_models)

    }
}

implement_repository!(
    DoctorAppointmentTypeRepository,
    DoctorAppointmentTypeModel,
    String
);
