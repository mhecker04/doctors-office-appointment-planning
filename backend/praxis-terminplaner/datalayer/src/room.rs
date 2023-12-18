use async_trait::async_trait;
use models::room::RoomModel;
use sea_orm::DatabaseConnection;

use crate::{base::{SeaOrmRepository, Repository}, implement_repository};

use crate::base::RepositoryError;

pub struct RoomRepository;


#[async_trait]
impl SeaOrmRepository<'_, entities::room::Entity, entities::room::ActiveModel, RoomModel, String> for RoomRepository {

    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    } 

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }

}

implement_repository!(RoomRepository, RoomModel, String);