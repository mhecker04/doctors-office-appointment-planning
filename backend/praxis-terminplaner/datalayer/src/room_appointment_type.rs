use async_trait::async_trait;
use entities::{appointment, appointment_type, room_appointment_type, room};
use models::room_appointment_type::RoomAppointmentTypeModel;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, ModelTrait, QueryFilter};

use crate::{
    base::{map_to_vector, ListRepository, Repository, map_to_model},
    error::RepositoryError,
    implement_repository,
    sea::SeaOrmRepository,
};

pub struct RoomAppointmentTypeRepository;

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        room_appointment_type::Entity,
        room_appointment_type::ActiveModel,
        RoomAppointmentTypeModel,
        String,
    > for RoomAppointmentTypeRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

#[async_trait]
impl ListRepository<RoomAppointmentTypeModel, String> for RoomAppointmentTypeRepository {
    async fn get_by_parent_id(
        &self,
        parent_id: &String,
    ) -> Result<Vec<RoomAppointmentTypeModel>, RepositoryError> {
        let connection = self.get_connection().await;

        let entities = room_appointment_type::Entity::find()
            .filter(room_appointment_type::Column::RoomId.eq(parent_id))
            .all(&connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        let related_appointment_types = entities
            .load_one(appointment_type::Entity, &connection)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        let room_appointment_type_models: Vec<RoomAppointmentTypeModel> = map_to_vector(&entities)?;

        let mut i = 0;

        let mut result_models: Vec<RoomAppointmentTypeModel> = Vec::new();

        for mut room_appointment_type_model in room_appointment_type_models {

            room_appointment_type_model.appointment_type = Some(map_to_model(related_appointment_types.get(i).unwrap())?);
            result_models.push(room_appointment_type_model);
            i += 1;
        }

        Ok(result_models)

    }
}

implement_repository!(
    RoomAppointmentTypeRepository,
    RoomAppointmentTypeModel,
    String
);
