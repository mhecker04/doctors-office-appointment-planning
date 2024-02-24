use business::{search::SearchBusiness, room::RoomBusiness, user::UserBusiness, appointment_type::AppointmentTypeBusiness};
use datalayer::{room::RoomRepository, user::UserRepository, appointment_type::AppointmentTypeRepository};
use models::{room::RoomModel, user::UserModel, appointment_type::AppointmentTypeModel};


use crate::request_guards::authentication::Token;
use rocket::{http::Status, response::status::Custom, serde::json::Json};

macro_rules! search_route {
    ($search_key:ident, $route:expr, $model_type:expr, $business:expr) => {
        #[get($route)]
        pub async fn $search_key(
            _token: Token,
            search_clause: &str,
        ) -> Custom<Result<Json<Vec<$model_type>>, &'static str>> {

            let search_models_result = $business.search( 
                &String::from(search_clause),
            )
            .await;

            match search_models_result {
                Ok(models) => Custom(Status::Ok, Ok(Json(models))),
                Err(_) => Custom(Status::InternalServerError, Err("search failed")),
            }
        }
    };

}

search_route!(room, "/room?<search_clause>", RoomModel, RoomBusiness{repository: RoomRepository});
search_route!(user, "/user?<search_clause>", UserModel, UserBusiness{repository: UserRepository});
search_route!(appointment_type, "/appointmentType?<search_clause>", AppointmentTypeModel, AppointmentTypeBusiness{repository: AppointmentTypeRepository});