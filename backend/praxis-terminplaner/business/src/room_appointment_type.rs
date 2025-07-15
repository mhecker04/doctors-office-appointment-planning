use datalayer::{error::RepositoryError, room_appointment_type::RoomAppointmentTypeRepository};
use models::room_appointment_type::RoomAppointmentTypeModel;

use crate::base::{Business, ListBusiness};

pub struct RoomAppointmentTypeBusiness {
    pub repository: RoomAppointmentTypeRepository,
}

impl Business<RoomAppointmentTypeRepository, RoomAppointmentTypeModel, String>
    for RoomAppointmentTypeBusiness
{
    fn get_repository(&self) -> &RoomAppointmentTypeRepository {
        &self.repository
    }
}

impl ListBusiness<RoomAppointmentTypeRepository, RoomAppointmentTypeModel, String>
    for RoomAppointmentTypeBusiness
{
}

impl RoomAppointmentTypeBusiness {
    pub async fn get_by_appointment_type_id(
        &self,
        appointment_type_id: &String,
    ) -> Result<Vec<RoomAppointmentTypeModel>, RepositoryError> {
        self.repository
            .get_by_appointment_type_id(appointment_type_id)
            .await
    }
}
