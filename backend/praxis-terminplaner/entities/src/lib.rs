
#[macro_use]
extern crate diesel;
extern crate dotenv;


pub mod schema;
pub mod user;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use uuid::Uuid;
use std::env;


pub fn establish_connection() -> MysqlConnection {

    let database_url = "mysql://root:admin@127.0.0.1:3306/terminplaner";

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))

}

pub trait Entity<TPrimaryKey> {
    fn get_primary_key(&self) -> &TPrimaryKey;
}