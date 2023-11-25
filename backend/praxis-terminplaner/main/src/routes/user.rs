use rocket::{serde::json::Json, response::status::Custom, http::Status};
use entities::user::User;

use business::base::Business;


use business::user::UserBusiness;
use datalayer::user::UserRepository;

use crate::request_guards::authentication::Token;

const business: UserBusiness = UserBusiness {
    repository: UserRepository{}
};


#[post("/", data="<user>")]
pub fn post_user(token: Token, user: Json<User>) -> Custom<Result<String, &'static str>> {

    let result = business.insert(&user);

    match result {
        Ok(v) => Custom(Status::Ok, Ok(String::from("hi"))),
        Err(e) => Custom(Status::InternalServerError, Err("sorry i fucked up"))
    }

}