use serde::{Serialize, Deserialize};

use crate::Model;



#[derive(Serialize, Deserialize, Clone)]
pub struct UserModel {
    pub user_id: Option<String>,
    pub username: String,
    pub password: String
}

impl Model<String> for UserModel {

    fn set_primary_key(&mut self, primary_key: &String) {
        self.user_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.user_id
    }
}
