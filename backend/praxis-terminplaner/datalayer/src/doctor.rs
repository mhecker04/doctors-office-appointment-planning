use async_trait::async_trait;
use entities::doctor_appointment_type::{self};
use futures::stream::iter;
use models::{doctor::DoctorModel, person::PersonModel};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, ModelTrait, QueryFilter};

use crate::{
    base::{map_to_model, map_to_vector, Repository},
    error::RepositoryError,
    implement_repository,
    sea::{map_sea_orm_error, SeaOrmRepository},
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
        let doctor_sea_model = SeaOrmRepository::get_entity(self, id).await?;

        let mut doctor_model: DoctorModel = map_to_model(&doctor_sea_model)?;

        let related_person_sea_model = doctor_sea_model
            .find_related(entities::person::Entity)
            .one(&self.get_connection().await)
            .await
            .map_err(map_sea_orm_error)?;

        match related_person_sea_model {
            Some(related_person_sea_model) => {
                let person_model: PersonModel = map_to_model(&related_person_sea_model)?;
                doctor_model.person = Some(person_model)
            }
            None => {}
        }
        Ok(doctor_model)
    }

    pub async fn load_doctors_qualified_for_appointment_type(
        &self,
        appointment_type_id: &String,
    ) -> Result<Vec<DoctorModel>, RepositoryError> {
        let connection = self.get_connection().await;

        let doctor_appointment_type_sea_models = doctor_appointment_type::Entity::find()
            .filter(
                entities::doctor_appointment_type::Column::AppointmentTypeId
                    .eq(appointment_type_id),
            )
            .all(&connection)
            .await
            .map_err(map_sea_orm_error)?;

        let optional_doctor_sea_models = doctor_appointment_type_sea_models
            .load_one(entities::doctor::Entity, &connection)
            .await
            .map_err(map_sea_orm_error)?;

        let mut doctor_sea_models: Vec<entities::doctor::Model> = Vec::new();

        for optional_doctor in optional_doctor_sea_models {
            if let Some(doctor) = optional_doctor {
                doctor_sea_models.push(doctor)
            }
        }

        let mut doctor_models = Vec::new();

        let person_sea_models = doctor_sea_models
            .load_one(entities::person::Entity, &connection)
            .await
            .map_err(map_sea_orm_error)?;

        for iterator in doctor_sea_models.into_iter().zip(person_sea_models) 
        {
            let (doctor_sea_model, person_sea_model)= iterator;
        
            match person_sea_model {
                Some(person_sea_model) => {
                    let mut doctor_model: DoctorModel = map_to_model(&doctor_sea_model)?;
                    doctor_model.person = map_to_model(&person_sea_model)?;
                    doctor_models.push(doctor_model);
                },
                _ => {}
            }
        }

        Ok(doctor_models)

    }
}

implement_repository!(DoctorRepository, DoctorModel, String);
