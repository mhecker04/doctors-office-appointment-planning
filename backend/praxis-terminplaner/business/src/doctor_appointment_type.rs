use datalayer::doctor_appointment_type::DoctorAppointmentTypeRepository;
use models::doctor_appointment_type::DoctorAppointmentTypeModel;

use crate::base::{Business, ListBusiness};

pub struct DoctorAppointmentTypeBusiness {
    pub repository: DoctorAppointmentTypeRepository
}

impl Business<DoctorAppointmentTypeRepository, DoctorAppointmentTypeModel, String> for DoctorAppointmentTypeBusiness {

    fn get_repository(&self) ->  &DoctorAppointmentTypeRepository {
        &self.repository
    }

}

impl ListBusiness<DoctorAppointmentTypeRepository, DoctorAppointmentTypeModel, String> for DoctorAppointmentTypeBusiness {

}
