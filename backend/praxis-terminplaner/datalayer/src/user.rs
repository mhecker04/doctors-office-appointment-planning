use async_trait::async_trait;
use models::user::UserModel;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};

use crate::{base::{SeaOrmRepository, RepositoryError, Repository}, implement_repository};
pub struct UserRepository;

impl UserRepository {
    pub async fn get_by_username(&self, username: &String) -> Result<UserModel, RepositoryError> {
        let result = entities::user::Entity::find()
            .filter(entities::user::Column::Username.eq(username))
            .one(&self.get_connection().await).await;

        match result {
            Ok(option) => match option {
                Some(model) => crate::base::map_to_model(&model),
                None => Err(RepositoryError::NoRecordFound)
            },
            Err(_) => Err(RepositoryError::NoConnection)
        }
    }
}

#[async_trait]
impl SeaOrmRepository<'_, entities::user::Entity, entities::user::ActiveModel, UserModel, String> for UserRepository {

    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    } 

    async fn get_connection(&self) -> DatabaseConnection {

        crate::base::get_mysql_connection().await
    }

}

implement_repository!(UserRepository, UserModel, String);