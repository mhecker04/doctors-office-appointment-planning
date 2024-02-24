use std::str::FromStr;

use business::{appointment::AppointmentBusiness, appointment_type::AppointmentTypeBusiness};
use chrono::NaiveDateTime;
use datalayer::{appointment::AppointmentRepository, appointment_type::AppointmentTypeRepository};
use datalayer::error::RepositoryError;
use models::appointment::AppointmentModel;
use models::available_appointment_resources::{AvailableAppointmentRessourcesModel, self};
use rocket::{serde::json::Json, response::status::Custom, http::Status};
use business::{self, base::Business};

use crate::{request_guards::authentication::Token, crud_endpoints};

use paste::paste;

use super::parse_result;

const BUSINESS: AppointmentBusiness = AppointmentBusiness{
    repository: AppointmentRepository,
    appointment_type_business: AppointmentTypeBusiness{
        repository: AppointmentTypeRepository
    }
};

crud_endpoints!(BUSINESS, AppointmentModel, appointment);


#[get("/resources?<appointment_type_id>&<datetime>")]
pub async fn get_available_ressources(_token: Token, appointment_type_id: &str, datetime: &str) ->
    Custom<Result<Json<AvailableAppointmentRessourcesModel>, &'static str>>
{

    let datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S%.fZ");

    match datetime {
        Ok(datetime) => {
            let available_appointment_resources_result = BUSINESS.get_available_ressources(&String::from(appointment_type_id), datetime).await;

            parse_result(available_appointment_resources_result)
            
        },
        Err(_) => {
            Custom(Status::BadRequest, Err("invalid datetime format"))
        }
    }    

}


