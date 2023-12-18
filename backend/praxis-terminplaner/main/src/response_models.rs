
extern crate rocket;

use rocket::Responder;
use rocket::serde::Serialize;


#[derive(Responder, Debug)]
pub enum NetworkResponse {
    #[response(status = 401)]
    Unauthorized(String),
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}