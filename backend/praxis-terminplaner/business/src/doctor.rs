use std::sync::Arc;

use async_trait::async_trait;
use datalayer::{
    doctor::DoctorRepository, doctor_appointment_type::DoctorAppointmentTypeRepository,
    error::RepositoryError, person,
};
use models::{doctor::DoctorModel, person::PersonModel};

use crate::{base::Business, person::PersonBusiness};

pub struct DoctorBusiness {
    pub repository: DoctorRepository,
    pub person_business: PersonBusiness,
}

#[async_trait]
impl Business<DoctorRepository, DoctorModel, String> for DoctorBusiness {
    fn get_repository(&self) -> &DoctorRepository {
        &self.repository
    }
}

impl DoctorBusiness {

    pub async fn create(&self, model: &mut DoctorModel) -> Result<String, RepositoryError> {

        let person_result = match model.person.as_mut() {
            Some(mut person) => self.person_business.create(&mut person).await,
            None => Err(RepositoryError::NoPersonSpecified),
        };

        let person_id = person_result?;
        model.person_id = Some(person_id);

        Business::create(self, model).await
    }

    pub async fn update(&self, model: &DoctorModel) -> Result<(), RepositoryError> {

        let person_result = match &model.person {
            Some(person) => self.person_business.update(person).await,
            None => Err(RepositoryError::NoPersonSpecified),
        };

        person_result?;

        Business::update(self, model).await

    }

    pub async fn get_by_id(&self, id: &String) -> Result<DoctorModel, RepositoryError> {

        self.repository.get_by_id(id).await

    }

}

