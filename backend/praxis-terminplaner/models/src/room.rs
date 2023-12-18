use serde::{Serialize, Deserialize};

use crate::Model;


#[derive(Deserialize, Serialize)]
pub struct RoomModel {

    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub room_number: Option<String>
}

impl Model<String> for RoomModel {

    fn set_primary_key(&mut self, primary_key: &String) {
        self.room_id = Some(primary_key.clone());
    }

    fn get_primary_key(&self) -> &Option<String> {
        &self.room_id
    }
}