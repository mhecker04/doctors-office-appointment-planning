use serde::{Deserialize, Serialize};

use crate::Model;




#[derive(Serialize, Deserialize)]
pub struct AppointmentTypeModel {

    pub appointment_type_id: Option<String>,
    pub appointment_type_name: String,
    pub duration: chrono::NaiveTime

}

impl Model<String> for AppointmentTypeModel {

    fn set_primary_key(&mut self, primary_key: &String) {
        self.appointment_type_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.appointment_type_id
    }

}