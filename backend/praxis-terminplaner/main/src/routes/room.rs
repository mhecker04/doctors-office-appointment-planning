use datalayer::room::RoomRepository;
use models::room::RoomModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status};
use business::{self, room::RoomBusiness, base::Business};

use crate::request_guards::authentication::Token;

const BUSINESS: RoomBusiness = RoomBusiness {
    repository: RoomRepository {}
};


#[post("/", data="<model>")]
pub async fn insert_room(token: Token, mut model: Json<RoomModel>) -> Custom<Result<String, &'static str>> {

    let result = BUSINESS.insert(&mut model).await;

    match result {
        Ok(v) => Custom(Status::Ok, Ok(v)),
        Err(_) => Custom(Status::InternalServerError, Err("failed to create user"))
    }

}
