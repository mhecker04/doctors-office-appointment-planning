use async_trait::async_trait;
use models::Model;
use std::marker::Send;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, Database, DatabaseConnection, EntityTrait,
    IntoActiveModel, PrimaryKeyTrait,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

    async fn delete(&self, id: &TPrimaryKey) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait SeaOrmRepository<'a, TEntity, TActiveModel, TModel, TPrimaryKey>
where
    TEntity: EntityTrait,
    <<TEntity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType:
        std::convert::From<&'a TPrimaryKey>,
    <TEntity as EntityTrait>::Model: IntoActiveModel<TActiveModel>,
    TActiveModel: ActiveModelTrait<Entity = TEntity> + ActiveModelBehavior + Send,
    <<TActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model:
        IntoActiveModel<TActiveModel>,
    for<'de> <<TActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model:
        serde::de::Deserialize<'de> + Serialize,
    TModel: Model<TPrimaryKey> + DeserializeOwned + Serialize + Send + Sync,
    TPrimaryKey: Into<<TEntity::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + 'a,
{
    fn create_new_primary_key(&self) -> TPrimaryKey;

    async fn get_connection(&self) -> DatabaseConnection;

    async fn create(&self, model: &mut TModel) -> Result<TPrimaryKey, RepositoryError> {
        let db = self.get_connection().await;

        let new_primary_key = self.create_new_primary_key();

        model.set_primary_key(&new_primary_key);

        let database_model = TActiveModel::from_json(serde_json::to_value(model).unwrap())
            .map_err(|_| RepositoryError::MappingError)?;

        let insert_result = database_model.insert(&db).await;

        match insert_result {
            Ok(_) => Ok(new_primary_key),
            Err(_) => Err(RepositoryError::NoConnection),
        }
    }

    async fn update(&self, model: &TModel) -> Result<(), RepositoryError> {
        let db = self.get_connection().await;

        let json_value = serde_json::to_value(model).map_err(|_| RepositoryError::MappingError)?;

        let database_model: TActiveModel =
            TActiveModel::from_json(json_value).map_err(|_| RepositoryError::MappingError)?;

        TEntity::update(database_model)
            .exec(&db)
            .await
            .map_err(|_| RepositoryError::NoConnection)?;

        Ok(())
    }

    async fn get_by_id(&self, id: &'a TPrimaryKey) -> Result<TModel, RepositoryError>
    where
        TPrimaryKey: 'a,
    {
        let db = self.get_connection().await;

        let model = TEntity::find_by_id(id).one(&db).await;

        match model {
            Ok(v) => match v {
                Some(m) => map_to_model(&m),
                None => Err(RepositoryError::NoRecordFound),
            },
            Err(_) => Err(RepositoryError::NoConnection),
        }
    }

    async fn delete(&self, id: &'a TPrimaryKey) -> Result<(), RepositoryError> {
        let db = self.get_connection().await;

        let delete_result = TEntity::delete_by_id(id).exec(&db).await;

        match delete_result {
            Err(_) => Err(RepositoryError::NoRecordFound),
            Ok(_) => Ok(()),
        }
    }
}

pub enum RepositoryError {
    NoConnection,
    NoRecordFound,
    MappingError,
}

pub fn map_to_model<TActiveModel, TModel>(source: &TActiveModel) -> Result<TModel, RepositoryError>
where
    TModel: DeserializeOwned,
    TActiveModel: Serialize,
{
    let json_value = serde_json::to_value(source).map_err(|_| RepositoryError::MappingError)?;

    serde_json::from_value(json_value).map_err(|_| RepositoryError::MappingError)
}


#[macro_export]
macro_rules! implement_repository {
    ($type:ty, $model:ty, $primary_key:ty) => {
        #[async_trait]
        impl Repository<$model, $primary_key> for $type {
            async fn create(
                &self,
                model: &mut $model,
            ) -> Result<$primary_key, RepositoryError> {
                SeaOrmRepository::create(self, model).await
            }

            async fn update(&self, model: &$model) -> Result<(), RepositoryError> {
                SeaOrmRepository::update(self, model).await
            }

            async fn get_by_id(
                &self,
                id: &$primary_key,
                ) -> Result<$model, RepositoryError> {
                SeaOrmRepository::get_by_id(self, id).await
            }

            async fn delete(&self, id: &$primary_key) -> Result<(), RepositoryError> {
                SeaOrmRepository::delete(self, id).await
            }
        }
    };
}
