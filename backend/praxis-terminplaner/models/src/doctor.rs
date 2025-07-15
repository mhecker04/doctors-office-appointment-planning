use serde::{Serialize, Deserialize};

use crate::{person::PersonModel, Model};

#[derive(Serialize, Deserialize, Clone)]
pub struct DoctorModel {
    pub doctor_id: Option<String>,
    pub person_id: Option<String>,
    pub person: Option<PersonModel>
}

impl Model<String> for DoctorModel {

    fn set_primary_key(&mut self, primary_key: &String) {
        self.doctor_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.doctor_id
    }

}
