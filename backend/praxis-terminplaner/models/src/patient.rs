use serde::{Serialize, Deserialize};

use crate::{person::PersonModel, Model};

#[derive(Serialize, Deserialize)]
pub struct PatientModel {

    patient_id: Option<String>,
    person_id: Option<String>,
    person: Option<PersonModel>

}

impl Model<String> for PatientModel {
    fn set_primary_key(&mut self, primary_key: &String) {
        self.patient_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.patient_id
    }

}