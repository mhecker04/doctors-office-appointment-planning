use serde::{Deserialize, Serialize};

use crate::{user::UserModel, Model};

#[derive(Serialize, Deserialize, Clone)]
pub struct PersonModel {
    pub person_id: Option<String>,
    pub lastname: Option<String>,
    pub firstname: Option<String>,
    pub email: Option<String>,
    pub user_id: Option<String>,
    pub user: Option<UserModel>,
}

impl Model<String> for PersonModel {
    fn set_primary_key(&mut self, primary_key: &String) {
        self.person_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.person_id
    }
}
