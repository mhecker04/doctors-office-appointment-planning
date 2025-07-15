use serde::{Deserialize, Serialize};

use crate::{Model, appointment_type::AppointmentTypeModel};



#[derive(Serialize, Deserialize, Clone)]
pub struct DoctorAppointmentTypeModel {

    pub doctor_appointment_type_id: Option<String>,
    pub appointment_type_id: String,
    pub doctor_id: String,
    pub appointment_type: Option<AppointmentTypeModel>

}

impl Model<String> for DoctorAppointmentTypeModel {
    
    fn set_primary_key(&mut self, primary_key: &String) {
        self.doctor_appointment_type_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.doctor_appointment_type_id
    }

}






