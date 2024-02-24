use chrono::naive::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::Model;

#[derive(Deserialize, Serialize)]
pub struct AppointmentModel {
    pub appointment_id: Option<String>,
    pub appointment_type_id: String,
    pub doctor_id: String,
    pub from: NaiveDateTime,
    pub patient_id: String,
    pub room_id: Option<String>,
}


impl Model<String> for AppointmentModel {

    fn set_primary_key(&mut self, primary_key: &String) {
        self.appointment_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.appointment_id
    }

}