use async_trait::async_trait;
use models::person::PersonModel;
use sea_orm::DatabaseConnection;

use crate::{
    base::Repository, error::RepositoryError, implement_repository, sea::SeaOrmRepository,
};

pub struct PersonRepository;

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        entities::person::Entity,
        entities::person::ActiveModel,
        PersonModel,
        String,
    > for PersonRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

implement_repository!(PersonRepository, PersonModel, String);
