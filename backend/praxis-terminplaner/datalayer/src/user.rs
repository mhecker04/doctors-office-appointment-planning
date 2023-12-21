use std::f32::consts::E;

use async_trait::async_trait;
use entities::user;
use models::user::UserModel;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    base::{map_to_model, Repository, map_to_vector},
    error::RepositoryError,
    implement_repository,
    sea::SeaOrmRepository,
    search::SearchRepository,
};
pub struct UserRepository;

impl UserRepository {
    pub async fn get_by_username(&self, username: &String) -> Result<UserModel, RepositoryError> {
        let result = entities::user::Entity::find()
            .filter(entities::user::Column::Username.eq(username))
            .one(&self.get_connection().await)
            .await;

        match result {
            Ok(option) => match option {
                Some(model) => crate::base::map_to_model(&model),
                None => Err(RepositoryError::NoRecordFound),
            },
            Err(_) => Err(RepositoryError::NoConnection),
        }
    }

}

#[async_trait]
impl SeaOrmRepository<'_, entities::user::Entity, entities::user::ActiveModel, UserModel, String>
    for UserRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

#[async_trait]
impl SearchRepository<UserModel, String> for UserRepository {
    async fn search(&self, search_clause: &String) -> Result<Vec<UserModel>, RepositoryError> {
        let search_predicate = entities::user::Column::Username.like(format!("%{}%", search_clause));

        let search_items = user::Entity::find()
            .filter(search_predicate)
            .all(&self.get_connection().await)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        map_to_vector(&search_items)

    }
}

implement_repository!(UserRepository, UserModel, String);
