use async_trait::async_trait;
use entities::{room, room_appointment_type};
use models::room::RoomModel;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};

use crate::base::map_to_vector;
use crate::sea::{get_search_predicate, map_sea_orm_error};
use crate::search::SearchRepository;
use crate::{base::Repository, implement_repository, sea::SeaOrmRepository};

use crate::error::RepositoryError;

pub struct RoomRepository;

#[async_trait]
impl SeaOrmRepository<'_, entities::room::Entity, entities::room::ActiveModel, RoomModel, String>
    for RoomRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

#[async_trait]
impl SearchRepository<RoomModel, String> for RoomRepository {

    async fn search(&self, search_clause: &String) -> Result<Vec<RoomModel>, RepositoryError> {

        let search_result = room::Entity::find()
            .filter(get_search_predicate(search_clause, vec![room::Column::RoomName, room::Column::RoomNumber]))
            .all(&self.get_connection().await).await.map_err(|_| RepositoryError::NoConnection)?;

        map_to_vector(&search_result)

    }


}

impl RoomRepository {

    pub async fn load_rooms_qualified_for_appointment_type(&self, appointment_type_id: &String) -> Result<Vec<RoomModel>, RepositoryError> {

        let connection = self.get_connection().await;

        let room_appointment_type_sea_models = entities::room_appointment_type::Entity::find()
            .filter(entities::room_appointment_type::Column::AppointmentTypeId.eq(appointment_type_id))
            .all(&connection)
            .await
            .map_err(map_sea_orm_error)?;

        let rooms = room_appointment_type_sea_models.load_one(entities::room::Entity, &connection)

            .await
            .map_err(map_sea_orm_error)?;

        map_to_vector(&rooms)

    }
}

implement_repository!(RoomRepository, RoomModel, String);
