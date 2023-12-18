use async_trait::async_trait;
use models::appointment_type::AppointmentTypeModel;
use sea_orm::DatabaseConnection;

use crate::{base::{SeaOrmRepository, Repository, RepositoryError}, implement_repository};



pub struct AppointmentTypeRepository;

#[async_trait]
impl SeaOrmRepository<'_, entities::appointment_type::Entity, entities::appointment_type::ActiveModel, AppointmentTypeModel, String> for AppointmentTypeRepository
{

    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    } 

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }

}

implement_repository!(AppointmentTypeRepository, AppointmentTypeModel, String);