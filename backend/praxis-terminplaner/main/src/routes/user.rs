use models::user::UserModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status, post};

use business::base::Business;


use business::user::UserBusiness;
use datalayer::user::UserRepository;

use crate::request_guards::authentication::Token;

const business: UserBusiness = UserBusiness {
    repository: UserRepository{}
};

#[post("/", data="<user>")]
pub async fn post_user(token: Token, mut user: Json<UserModel>) -> Custom<Result<String, &'static str>> {

    let result = business.insert(&mut user).await;

    match result {
        Ok(v) => Custom(Status::Ok, Ok(v)),
        Err(e) => Custom(Status::InternalServerError, Err("failed to create user"))
    }

}