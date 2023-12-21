use async_trait::async_trait;
use entities::appointment_type;
use futures::TryFutureExt;
use models::appointment_type::AppointmentTypeModel;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    base::{Repository, map_to_vector}, error::RepositoryError, implement_repository, sea::{SeaOrmRepository, get_search_predicate}, search::SearchRepository,
};

pub struct AppointmentTypeRepository;

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        entities::appointment_type::Entity,
        entities::appointment_type::ActiveModel,
        AppointmentTypeModel,
        String,
    > for AppointmentTypeRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

#[async_trait]
impl SearchRepository<AppointmentTypeModel, String> for AppointmentTypeRepository {

    async fn search(&self, search_clause: &String) -> Result<Vec<AppointmentTypeModel>, RepositoryError> {

        let search_items = appointment_type::Entity::find()
            .filter(get_search_predicate(search_clause, vec![appointment_type::Column::AppointmentTypeName]))
            .all(&self.get_connection().await).await.map_err(|_| RepositoryError::NoConnection)?;

        map_to_vector(&search_items)

    }

}

implement_repository!(AppointmentTypeRepository, AppointmentTypeModel, String);
