use business::doctor::DoctorBusiness;
use business::person::PersonBusiness;
use business::room::RoomBusiness;
use business::{self, base::Business};
use business::{appointment::AppointmentBusiness, appointment_type::AppointmentTypeBusiness};
use chrono::NaiveDateTime;
use datalayer::doctor::DoctorRepository;
use datalayer::person::PersonRepository;
use datalayer::room::RoomRepository;
use datalayer::{appointment::AppointmentRepository, appointment_type::AppointmentTypeRepository};
use models::appointment::AppointmentModel;
use models::available_appointment_resources::AvailableAppointmentRessourcesModel;
use models::possible_appointment::{PosssibleAppointmentModel};
use rocket::{http::Status, response::status::Custom, serde::json::Json};

use crate::{crud_endpoints, parse_to_json_response, request_guards::authentication::Token};

use paste::paste;

use super::parse_result;

const BUSINESS: AppointmentBusiness = AppointmentBusiness {
    repository: AppointmentRepository,
    appointment_type_business: AppointmentTypeBusiness {
        repository: AppointmentTypeRepository,
    },
    doctor_business: DoctorBusiness {
        repository: DoctorRepository,
        person_business: PersonBusiness {
            repository: PersonRepository
        }
    },
    room_business: RoomBusiness {
        repository: RoomRepository,
    },
};

crud_endpoints!(BUSINESS, AppointmentModel, appointment);

#[get("/resources?<appointment_type_id>&<datetime>")]
pub async fn get_available_ressources(
    _token: Token,
    appointment_type_id: &str,
    datetime: &str,
) -> Custom<Result<Json<AvailableAppointmentRessourcesModel>, &'static str>> {
    let datetime = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S%.fZ");

    match datetime {
        Ok(datetime) => {
            let available_appointment_resources_result = BUSINESS
                .get_available_ressources(&String::from(appointment_type_id), datetime)
                .await;

            parse_result(available_appointment_resources_result)
        }
        Err(_) => Custom(Status::BadRequest, Err("invalid datetime format")),
    }
}

#[get("/possible?<appointment_type_id>&<from>&<to>")] 
pub async fn get_possible_appointments(
    _token: Token,
    appointment_type_id: &str,
    from: &str,
    to: &str
) -> Custom<Result<Json<Vec<PosssibleAppointmentModel>>, &'static str>> {

    let from = NaiveDateTime::parse_from_str(from, "%Y-%m-%dT%H:%M:%S%.fZ");
    let to = NaiveDateTime::parse_from_str(to, "%Y-%m-%dT%H:%M:%S%.fZ");

    match (from, to) {
        (Ok(from), Ok(to)) => {
            let possible_appointments_result = BUSINESS.get_possible_appointments(&String::from(appointment_type_id), from, to).await;
            parse_result(possible_appointments_result)
        },
        (_, _) => Custom(Status::BadRequest, Err("invalid datetime format")),
    }
}

