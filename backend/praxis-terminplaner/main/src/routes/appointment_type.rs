use business::appointment_type::AppointmentTypeBusiness;
use business::base::Business;
use datalayer::{appointment_type::AppointmentTypeRepository, error::RepositoryError};
use models::appointment_type::AppointmentTypeModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status};

use crate::{request_guards::authentication::Token, crud_endpoints, parse_to_json_response};
use paste::paste;




const BUSINESS: AppointmentTypeBusiness = AppointmentTypeBusiness {
    repository: AppointmentTypeRepository {}
};

crud_endpoints!(BUSINESS, AppointmentTypeModel, appointment_type);
