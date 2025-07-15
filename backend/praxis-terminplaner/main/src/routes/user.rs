use models::user::UserModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status, post};

use business::base::Business;

use business::user::UserBusiness;
use datalayer::{user::UserRepository, error::RepositoryError};

use crate::{request_guards::authentication::Token, crud_endpoints, parse_to_json_response};

use paste::paste;


const BUSINESS: UserBusiness = UserBusiness {
    repository: UserRepository{}
};

crud_endpoints!(BUSINESS, UserModel, user);
