use serde::{Deserialize, Serialize};

use crate::{Model, appointment_type::AppointmentTypeModel};


#[derive(Serialize, Deserialize)]
pub struct RoomAppointmentTypeModel {

    room_appointment_type_id: Option<String>,
    appointment_type_id: String,
    room_id: String ,
    pub appointment_type: Option<AppointmentTypeModel>

}

impl Model<String> for RoomAppointmentTypeModel {

    fn set_primary_key(&mut self, primary_key: &String) {
        self.room_appointment_type_id = Some(primary_key.clone())
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.room_appointment_type_id
    }
}

