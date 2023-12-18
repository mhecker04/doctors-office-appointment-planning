use business::appointment_type::AppointmentTypeBusiness;
use business::base::Business;
use datalayer::appointment_type::AppointmentTypeRepository;
use models::appointment_type::AppointmentTypeModel;
use rocket::{serde::json::Json, response::{status::Custom}, http::Status};

use crate::request_guards::authentication::Token;



const BUSINESS: AppointmentTypeBusiness = AppointmentTypeBusiness {
    repository: AppointmentTypeRepository {}
};

#[post("/", data="<model>")]
pub async fn insert_appointment_type(token: Token, mut model: Json<AppointmentTypeModel>) -> Custom<Result<String, &'static str>> {

    let result = BUSINESS.insert(&mut model).await;

    match result {
        Ok(v) => Custom(Status::Ok, Ok(v)),
        Err(_) => Custom(Status::InternalServerError, Err("failed to create user"))
    }

}