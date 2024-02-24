use async_trait::async_trait;
use chrono::NaiveDateTime;
use datalayer::{appointment::AppointmentRepository, error::RepositoryError};
use models::{appointment::AppointmentModel, available_appointment_resources::AvailableAppointmentRessourcesModel};

use crate::{base::Business, appointment_type::AppointmentTypeBusiness};

pub struct AppointmentBusiness {
    pub repository: AppointmentRepository,
    pub appointment_type_business: AppointmentTypeBusiness
}

#[async_trait]
impl Business<AppointmentRepository, AppointmentModel, String> for AppointmentBusiness {

    fn get_repository(&self) -> &AppointmentRepository {
        &self.repository
    }

}

impl AppointmentBusiness {

    pub async fn get_available_ressources(&self, appointment_type_id: &String, datetime: NaiveDateTime) -> Result<AvailableAppointmentRessourcesModel, RepositoryError> {
        
        let appointment_type = self.appointment_type_business.get_by_id(appointment_type_id).await?;

        self.repository.get_available_ressources(datetime, &appointment_type).await

    }

}
 
