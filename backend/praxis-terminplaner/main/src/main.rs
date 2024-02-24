#[macro_use]
extern crate rocket;

use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::launch;
use rocket::Request;
use rocket::Response;
use routes::appointment;
use routes::appointment_type;
use routes::auth;

mod request_guards;
mod response_models;
mod routes;

use routes::doctor;
use routes::doctor_appointment_type;
use routes::room;
use routes::room_appointment_type;
use routes::search;
use routes::user;

// Fairing to set the Access-Control-Allow-Origin header
struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, OPTIONS, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn options() {}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/auth", routes![auth::login])
        .mount(
            "/user",
            routes![
                user::create_user,
                user::update_user,
                user::get_user,
                user::delete_user
            ],
        )
        .mount(
            "/appointmentType",
            routes![
                appointment_type::create_appointment_type,
                appointment_type::update_appointment_type,
                appointment_type::get_appointment_type,
                appointment_type::delete_appointment_type
            ],
        )
        .mount(
            "/room",
            routes![
                room::create_room,
                room::get_room,
                room::update_room,
                room::delete_room,
                room_appointment_type::create_room_appointment_type,
                room_appointment_type::get_room_appointment_types
            ],
        )
        .mount(
            "/doctor",
            routes![
                doctor::create_doctor,
                doctor::get_doctor,
                doctor::update_doctor,
                doctor::delete_doctor,
                doctor_appointment_type::create_doctor_appointment_type,
                doctor_appointment_type::get_doctor_appointment_types
            ],
        )
        .mount(
            "/search",
            routes![search::user, search::appointment_type, search::room],
        )
        .mount(
            "/appointment",
            routes![
                appointment::create_appointment,
                appointment::get_appointment,
                appointment::update_appointment,
                appointment::delete_appointment,
                appointment::get_available_ressources
            ],
        )
        .mount("/", routes![options])
}

#[macro_export]
macro_rules! crud_endpoints {
    ($business:expr, $model_type:ty, $model_name:ident) => {
        paste! {
                #[post("/", data = "<model>")]
                pub async fn [<create_ $model_name>](
                    _token: Token,
                    mut model: Json<$model_type>,
                ) -> Custom<Result<Json<String>, &'static str>> {
                    let result = $business.create(&mut model).await;

                    match result {
                        Ok(v) => Custom(Status::Ok, Ok(Json(v))),
                        Err(_) => Custom(Status::InternalServerError, Err("failed to create")),
                    }
                }
            }
        paste! {

                #[get("/<id>")]
                pub async fn [<get_ $model_name>](
                    _token: Token,
                    id: &str
                ) -> Custom<Result<Json<$model_type>, &'static str>> {
                    let result = $business.get_by_id(&String::from(id)).await;

                    match result {
                        Ok(model) => Custom(Status::Ok, Ok(Json(model))),
                        Err(RepositoryError::NoRecordFound) => Custom(Status::NotFound, Err("no record found")),
                        Err(_) => Custom(Status::InternalServerError, Err("failed to get record"))
                    }
                }

            }

        paste! {



                #[put("/", data= "<model>")]
                pub async fn [<update_ $model_name>](
                    _token: Token,
                    model: Json<$model_type>
                ) -> Custom<Result<(), &'static str>> {

                    let result = $business.update(&model).await;

                    match result {
                        Ok(()) => Custom(Status::Ok, Ok(())),
                        Err(_) => Custom(Status::InternalServerError, Err("failed to update record"))
                    }
                }

        }

        paste! {
                #[delete("/<id>")]
                pub async fn [<delete_ $model_name>](
                    _token: Token,
                    id: &str
                ) -> Custom<Result<Json<String>, &'static str>> {
                    let result = $business.delete(&String::from(id)).await;

                    match result {
                        Ok(id) => Custom(Status::Ok, Ok(Json(id))),
                        Err(_) => Custom(Status::NotFound, Err("record could not be deleted"))
                    }

                }

        }
    };
}
