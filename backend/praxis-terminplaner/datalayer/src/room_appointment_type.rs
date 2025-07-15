use async_trait::async_trait;
use entities::{appointment_type, room_appointment_type};
use models::room_appointment_type::RoomAppointmentTypeModel;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};

use crate::{
    base::{map_to_model, map_to_vector, ListRepository, Repository},
    error::RepositoryError,
    implement_repository,
    sea::{map_sea_orm_error, SeaOrmRepository},
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

impl RoomAppointmentTypeRepository {
    pub async fn get_by_appointment_type_id(
        &self,
        appointment_type_id: &String,
    ) -> Result<Vec<RoomAppointmentTypeModel>, RepositoryError> {

        let connection = self.get_connection().await;

        let room_appointment_type_sea_models = entities::room_appointment_type::Entity::find()
            .filter(
                entities::room_appointment_type::Column::AppointmentTypeId
                    .eq(appointment_type_id),
            )
            .all(&connection)
            .await
            .map_err(map_sea_orm_error)?;


        map_to_vector(&room_appointment_type_sea_models)

    }


}

implement_repository!(
    RoomAppointmentTypeRepository,
    RoomAppointmentTypeModel,
    String
);
