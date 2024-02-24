use async_trait::async_trait;
use models::patient::PatientModel;
use sea_orm::DatabaseConnection;

use crate::{sea::SeaOrmRepository, implement_repository, base::Repository, error::RepositoryError};

pub struct PatientRepository {}

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        entities::patient::Entity,
        entities::patient::ActiveModel,
        PatientModel,
        String,
    > for PatientRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

implement_repository!(PatientRepository, PatientModel, String);