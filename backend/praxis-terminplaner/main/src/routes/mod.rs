use datalayer::error::RepositoryError;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use serde::Serialize;

pub mod appointment;
pub mod appointment_type;
pub mod auth;
pub mod doctor;
pub mod doctor_appointment_type;
pub mod room;
pub mod room_appointment_type;
pub mod search;
pub mod user;
pub mod patient;

pub fn parse_result<TModel>(
    result: Result<TModel, RepositoryError>,
) -> Custom<Result<Json<TModel>, &'static str>>
where
    TModel: Serialize,
{

    match result {
        Ok(model) => Custom(Status::Ok, Ok(Json(model))),
        Err(RepositoryError::MappingError) => Custom(
            Status::InternalServerError,
            Err("something went wrong while mapping"),
        ),
        Err(RepositoryError::NoConnection) => Custom(
            Status::InternalServerError,
            Err("connection to the database could not be established")
        ),
        Err(RepositoryError::NoPersonSpecified) => Custom(
            Status::BadRequest,
            Err("you didn't specify a person")
        ),
        Err(RepositoryError::InvalidDateTimeFormat) => Custom(
            Status::BadRequest,
            Err("invalid date time format")
        ),
        Err(RepositoryError::NoRecordFound) => Custom(
            Status::NotFound,
            Err("no record was found")
        ),
        Err(RepositoryError::DateOutOfRange) => Custom(
            Status::NotFound,
            Err("Date out of range")
        )
    }

}
