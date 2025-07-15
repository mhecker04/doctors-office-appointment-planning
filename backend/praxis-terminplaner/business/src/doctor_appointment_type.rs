use datalayer::{doctor_appointment_type::DoctorAppointmentTypeRepository, error::RepositoryError};
use models::doctor_appointment_type::DoctorAppointmentTypeModel;

use crate::base::{Business, ListBusiness};

pub struct DoctorAppointmentTypeBusiness {
    pub repository: DoctorAppointmentTypeRepository,
}

impl Business<DoctorAppointmentTypeRepository, DoctorAppointmentTypeModel, String>
    for DoctorAppointmentTypeBusiness
{
    fn get_repository(&self) -> &DoctorAppointmentTypeRepository {
        &self.repository
    }
}

impl ListBusiness<DoctorAppointmentTypeRepository, DoctorAppointmentTypeModel, String>
    for DoctorAppointmentTypeBusiness
{
}

impl DoctorAppointmentTypeBusiness {
    pub async fn get_by_appointment_type_id(
        &self,
        appointment_type_id: &String,
    ) -> Result<Vec<DoctorAppointmentTypeModel>, RepositoryError> {
        self.repository
            .get_by_appointment_type_id(appointment_type_id)
            .await
    }
}
