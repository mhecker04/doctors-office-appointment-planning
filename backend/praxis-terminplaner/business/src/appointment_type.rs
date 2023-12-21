use crate::{base::Business, implement_search_business, search::SearchBusiness};
use async_trait::async_trait;
use datalayer::appointment_type::AppointmentTypeRepository;
use datalayer::error::RepositoryError;
use datalayer::search::SearchRepository;
use models::appointment_type::AppointmentTypeModel;

pub struct AppointmentTypeBusiness {
    pub repository: AppointmentTypeRepository,
}

impl Business<AppointmentTypeRepository, AppointmentTypeModel, String> for AppointmentTypeBusiness {
    fn get_repository(&self) -> &AppointmentTypeRepository {
        &self.repository
    }
}

implement_search_business!(AppointmentTypeBusiness, AppointmentTypeModel, String);
