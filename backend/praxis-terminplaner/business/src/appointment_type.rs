use datalayer::appointment_type::AppointmentTypeRepository;
use models::appointment_type::AppointmentTypeModel;

use crate::base::Business;


pub struct AppointmentTypeBusiness {
    pub repository: AppointmentTypeRepository
}

impl Business<AppointmentTypeRepository, AppointmentTypeModel, String> for AppointmentTypeBusiness {
    
    fn get_repository(&self) -> &AppointmentTypeRepository {
        &self.repository
    }

}
