use business::base::Business;
use business::{doctor::DoctorBusiness, person::PersonBusiness};
use datalayer::{doctor::DoctorRepository, person::PersonRepository};

use datalayer::error::RepositoryError;
use models::doctor::DoctorModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status};
use business::{self};

use crate::{request_guards::authentication::Token, crud_endpoints, parse_to_json_response};

use paste::paste;

const BUSINESS: DoctorBusiness = DoctorBusiness{repository: DoctorRepository, person_business: PersonBusiness{repository: PersonRepository}};

crud_endpoints!(BUSINESS, DoctorModel, doctor);
