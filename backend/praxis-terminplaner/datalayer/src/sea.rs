use async_trait::async_trait;
use models::Model;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait,
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, LoaderTrait, ModelTrait,
    PrimaryKeyTrait, Related,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{base::map_to_model, error::RepositoryError};

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
    TPrimaryKey:
        Into<<TEntity::PrimaryKey as PrimaryKeyTrait>::ValueType> + Send + Sync + 'a + Clone,
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
            Err(err) => {
                println!("{}", err);
                return Err(RepositoryError::NoConnection)
            },
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
        let database_model = self.get_entity(id).await?;

        map_to_model(&database_model)
    }

    async fn get_entity(
        &self,
        id: &'a TPrimaryKey,
    ) -> Result<<TEntity as EntityTrait>::Model, RepositoryError> {
        let db = self.get_connection().await;

        let model = TEntity::find_by_id(id).one(&db).await;

        match model {
            Ok(v) => match v {
                Some(m) => Ok(m),
                None => Err(RepositoryError::NoRecordFound),
            },
            Err(_) => Err(RepositoryError::NoConnection),
        }
    }

    async fn delete(&self, id: &'a TPrimaryKey) -> Result<TPrimaryKey, RepositoryError> {
        let db = self.get_connection().await;

        let delete_result = TEntity::delete_by_id(id).exec(&db).await;

        match delete_result {
            Err(_) => Err(RepositoryError::NoRecordFound),
            Ok(_) => Ok(id.clone()),
        }
    }
}

pub fn get_search_predicate<TColumn>(search_clause: &String, columns: Vec<TColumn>) -> Condition
where
    TColumn: ColumnTrait,
{
    let mut predicate = Condition::any();

    for column in columns {
        predicate = predicate.add(column.like(format!("%{}%", search_clause)));
    }

    predicate
}

pub fn map_sea_orm_error(_err: DbErr) -> RepositoryError {
    RepositoryError::NoConnection
}
