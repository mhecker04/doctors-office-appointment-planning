use business::{doctor_appointment_type::DoctorAppointmentTypeBusiness, base::{ListBusiness, Business}};
use datalayer::doctor_appointment_type::DoctorAppointmentTypeRepository;
use models::doctor_appointment_type::DoctorAppointmentTypeModel;
use rocket::{serde::json::Json, http::Status, response::status::Custom};

use crate::request_guards::authentication::Token;





const BUSINESS: DoctorAppointmentTypeBusiness = DoctorAppointmentTypeBusiness{
    repository: DoctorAppointmentTypeRepository,
};

#[get("/<doctor_id>/appointmentTypes")]
pub async fn get_doctor_appointment_types(
    _token: Token,
    doctor_id: &str,
) -> Custom<Result<Json<Vec<DoctorAppointmentTypeModel>>, &'static str>> {
    let models_result = BUSINESS.get_by_parent_id(&String::from(doctor_id)).await;

    match models_result {
        Ok(models) => Custom(Status::Ok, Ok(Json(models))),
        Err(_) => Custom(
            Status::InternalServerError,
            Err("an unexpected error occured"),
        ),
    }
}

#[post("/<_doctor_id>/appointmentTypes", data = "<model>")]
pub async fn create_doctor_appointment_type(
    _token: Token,
    mut model: Json<DoctorAppointmentTypeModel>,
    _doctor_id: &str
) -> Custom<Result<String, &'static str>> {
    let result = BUSINESS.create(&mut model).await;

    match result {
        Ok(id) => Custom(Status::Ok, Ok(id)),
        Err(_) => Custom(Status::InternalServerError, Err("failed to create")),
    }

}
