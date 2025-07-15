use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use crate::Model;
use tools::datetime::deserialize_naive_time;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppointmentTypeModel {
    pub appointment_type_id: Option<String>,
    pub appointment_type_name: String,
    #[serde(deserialize_with = "deserialize_naive_time")]
    pub duration: NaiveTime,
}

impl Model<String> for AppointmentTypeModel {
    fn set_primary_key(&mut self, primary_key: &String) {
        self.appointment_type_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.appointment_type_id
    }
}


