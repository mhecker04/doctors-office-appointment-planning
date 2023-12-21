use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::Model;

#[derive(Serialize, Deserialize)]
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

fn deserialize_naive_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    // Define multiple time formats to attempt parsing
    let formats = ["%I:%M %p", "%H:%M:%S"];

    for format in formats {
        let result = NaiveTime::parse_from_str(s, format);
        match result {
            Ok(time) => return Ok(time),
            _ => {} 
        }            
    }

    Err(serde::de::Error::custom("Failed to parse NaiveTime"))
}
