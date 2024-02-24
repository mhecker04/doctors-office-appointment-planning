use models::user::UserModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status, post};

use business::base::Business;

use business::user::UserBusiness;
use datalayer::{user::UserRepository, error::RepositoryError};

use crate::{request_guards::authentication::Token, crud_endpoints};

use paste::paste;


const business: UserBusiness = UserBusiness {
    repository: UserRepository{}
};

crud_endpoints!(business, UserModel, user);
