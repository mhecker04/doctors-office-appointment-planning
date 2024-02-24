use async_trait::async_trait;
use entities::person;
use models::doctor::DoctorModel;
use sea_orm::{DatabaseConnection, ModelTrait};

use crate::{
    base::{Repository, map_to_model}, error::RepositoryError, implement_repository,
    sea::SeaOrmRepository,
};

pub struct DoctorRepository;

#[async_trait]
impl
    SeaOrmRepository<
        '_,
        entities::doctor::Entity,
        entities::doctor::ActiveModel,
        DoctorModel,
        String,
    > for DoctorRepository
{
    fn create_new_primary_key(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    async fn get_connection(&self) -> DatabaseConnection {
        crate::base::get_mysql_connection().await
    }
}

impl DoctorRepository {

    pub async fn get_by_id(&self, id: &String) -> Result<DoctorModel, RepositoryError> {

        let doctor_model = SeaOrmRepository::get_entity(self, id).await?;

        let related_person = doctor_model.find_related(person::Entity)
            .one(&self.get_connection().await)
            .await
            .map_err(|_| RepositoryError::NoPersonSpecified)?;


        let mut doctor_model: DoctorModel = map_to_model(&doctor_model)?;

        match related_person {
            Some(person) => {
                doctor_model.person = Some(map_to_model(&person)?);
                Ok(doctor_model)
            },
            None => {
                Err(RepositoryError::NoPersonSpecified)
            }
        }

    }

}


implement_repository!(DoctorRepository, DoctorModel, String);
