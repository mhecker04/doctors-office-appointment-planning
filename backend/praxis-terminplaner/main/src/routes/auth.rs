
use authentication::authorize;
use rocket::{serde::json::Json, response::status::Custom, http::Status};
use serde::{Deserialize, Serialize};

use crate::request_guards::authentication::Token;



#[derive(Deserialize)]
pub struct LoginParameters {
    pub username: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: usize
}

#[post("/login", data = "<login_parameters>")]
pub async fn login(login_parameters: Json<LoginParameters>) -> Custom<Result<Json<AccessToken>, &'static str>> {
    let token_result = authorize(&login_parameters.username, &login_parameters.password).await;

    match token_result {
        Ok(token) => {
            let access_token = AccessToken {
                access_token: token, expires_in: 300
            };
            Custom(Status::Ok, Ok(Json(access_token)))
        },
        Err(_) => Custom(Status::Unauthorized, Err("invalid credentials"))
    }

}
