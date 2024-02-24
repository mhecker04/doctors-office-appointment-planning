use datalayer::{room::RoomRepository, error::RepositoryError};
use models::room::RoomModel;
use rocket::{serde::json::Json, response::status::Custom, http::Status};
use business::{self, room::RoomBusiness, base::Business};

use crate::{request_guards::authentication::Token, crud_endpoints};

use paste::paste;

const BUSINESS: RoomBusiness = RoomBusiness {
    repository: RoomRepository {}
};

crud_endpoints!(BUSINESS, RoomModel, room);
