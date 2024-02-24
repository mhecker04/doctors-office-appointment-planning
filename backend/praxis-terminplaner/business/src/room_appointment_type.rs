use datalayer::room_appointment_type::RoomAppointmentTypeRepository;
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
