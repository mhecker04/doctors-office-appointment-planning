use business::{room_appointment_type::RoomAppointmentTypeBusiness, base::{Business, ListBusiness}};
use datalayer::room_appointment_type::RoomAppointmentTypeRepository;
use models::room_appointment_type::RoomAppointmentTypeModel;
use rocket::{response::status::Custom, serde::json::Json, http::Status};

use crate::request_guards::authentication::Token;

const BUSINESS: RoomAppointmentTypeBusiness = RoomAppointmentTypeBusiness {
    repository: RoomAppointmentTypeRepository,
};


#[get("/<room_id>/appointmentTypes")]
pub async fn get_room_appointment_types(
    _token: Token,
    room_id: &str,
) -> Custom<Result<Json<Vec<RoomAppointmentTypeModel>>, &'static str>> {
    let models_result = BUSINESS.get_by_parent_id(&String::from(room_id)).await;

    match models_result {
        Ok(models) => Custom(Status::Ok, Ok(Json(models))),
        Err(_) => Custom(
            Status::InternalServerError,
            Err("an unexpected error occured"),
        ),
    }
}

#[post("/<room_id>/appointmentTypes", data = "<model>")]
pub async fn create_room_appointment_type(
    _token: Token,
    mut model: Json<RoomAppointmentTypeModel>,
    room_id: &str
) -> Custom<Result<String, &'static str>> {
    let result = BUSINESS.create(&mut model).await;

    match result {
        Ok(id) => Custom(Status::Ok, Ok(id)),
        Err(_) => Custom(Status::InternalServerError, Err("failed to create")),
    }
}
