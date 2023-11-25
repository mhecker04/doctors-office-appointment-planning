use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Entity;


#[derive(Queryable, Selectable, Deserialize, Insertable, Clone, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::user)]
pub struct User {
    pub user_id: String,
    pub username: Option<String>,
    pub password: Option<String>
}


impl Entity<String> for User {

    fn get_primary_key(&self) -> &String {
        &self.user_id
    }
    
}