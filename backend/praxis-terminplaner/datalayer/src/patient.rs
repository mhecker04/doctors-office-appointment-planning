use async_trait::async_trait;
use models::patient::PatientModel;
use sea_orm::{sea_query::Query, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};

use crate::{
    base::{map_to_model, Repository},
    error::RepositoryError,
    implement_repository,
    sea::{map_sea_orm_error, SeaOrmRepository},
};

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

impl PatientRepository {

    pub async fn get_by_id(&self, id: &String) -> Result<PatientModel, RepositoryError> {
        let patient_sea_model: entities::patient::Model =
            SeaOrmRepository::get_entity(self, id).await?;

        self.map_to_model(patient_sea_model).await
    }

    pub async fn get_by_user_id(&self, user_id: &String) -> Result<PatientModel, RepositoryError> {
        let patient_sea_model = entities::patient::Entity::find()
            .filter(
                entities::patient::Column::PersonId.in_subquery(
                    Query::select()
                        .column(entities::person::Column::PersonId)
                        .from(entities::person::Entity)
                        .cond_where(entities::person::Column::UserId.eq(user_id))
                        .to_owned(),
                ),
            )
            .one(&self.get_connection().await)
            .await
            .map_err(map_sea_orm_error)?;

        match patient_sea_model  {
            Some(patient_sea_model) => self.map_to_model(patient_sea_model).await,
            None => Err(RepositoryError::NoRecordFound)
        }

    }

    async fn map_to_model(
        &self,
        patient_sea_model: entities::patient::Model,
    ) -> Result<PatientModel, RepositoryError> {
        let mut patient_model: PatientModel = map_to_model(&patient_sea_model)?;

        let person_sea_model = patient_sea_model
            .find_related(entities::person::Entity)
            .one(&self.get_connection().await)
            .await
            .map_err(map_sea_orm_error)?;

        match person_sea_model {
            Some(person_sea_model) => {
                let person_model = map_to_model(&person_sea_model)?;
                patient_model.person = Some(person_model);
            }
            None => {}
        }

        Ok(patient_model)
    }
}
