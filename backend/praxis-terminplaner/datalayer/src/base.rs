use async_trait::async_trait;

use sea_orm::{Database, DatabaseConnection};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::RepositoryError;

pub async fn get_mysql_connection() -> DatabaseConnection {
    Database::connect("mysql://root:admin@127.0.0.1:3306/appointment_planning")
        .await
        .unwrap()
}

#[async_trait]
pub trait Repository<TModel, TPrimaryKey> {
    async fn create(&self, model: &mut TModel) -> Result<TPrimaryKey, RepositoryError>;

    async fn update(&self, model: &TModel) -> Result<(), RepositoryError>;

    async fn get_by_id(&self, id: &TPrimaryKey) -> Result<TModel, RepositoryError>;

    async fn delete(&self, id: &TPrimaryKey) -> Result<TPrimaryKey, RepositoryError>;
}

#[async_trait]
pub trait ListRepository<TModel, TPrimaryKey>: Repository<TModel, TPrimaryKey> {
    async fn get_by_parent_id(&self, id: &TPrimaryKey) -> Result<Vec<TModel>, RepositoryError>;
}

pub fn map_to_model<TSource, TModel>(source: &TSource) -> Result<TModel, RepositoryError>
where
    TModel: DeserializeOwned,
    TSource: Serialize,
{
    let json_value = serde_json::to_string(source).map_err(|_| RepositoryError::MappingError)?;

    println!("json {}", json_value.as_str());

    serde_json::from_str(json_value.as_str()).map_err(|_| RepositoryError::MappingError)
}

pub fn map_to_vector<TSource, TDestination>(
    source: &Vec<TSource>,
) -> Result<Vec<TDestination>, RepositoryError>
where
    TDestination: DeserializeOwned,
    TSource: Serialize,
{
    let mut result = Vec::new();

    for item in source {
        result.push(map_to_model(&item)?)
    }

    Ok(result)
}

#[macro_export]
macro_rules! implement_repository {
    ($type:ty, $model:ty, $primary_key:ty) => {
        #[async_trait]
        impl Repository<$model, $primary_key> for $type {
            async fn create(&self, model: &mut $model) -> Result<$primary_key, RepositoryError> {
                SeaOrmRepository::create(self, model).await
            }

            async fn update(&self, model: &$model) -> Result<(), RepositoryError> {
                SeaOrmRepository::update(self, model).await
            }

            async fn get_by_id(&self, id: &$primary_key) -> Result<$model, RepositoryError> {
                SeaOrmRepository::get_by_id(self, id).await
            }

            async fn delete(&self, id: &$primary_key) -> Result<$primary_key, RepositoryError> {
                SeaOrmRepository::delete(self, id).await
            }
        }
    };
}
