#[macro_use] extern crate rocket;

use rocket::Data;
use rocket::Request;
use rocket::Response;
use rocket::fairing::Fairing;
use rocket::fairing::Info;
use rocket::fairing::Kind;
use rocket::http::Header;
use rocket::launch;
use routes::auth;

mod request_guards;
mod response_models;
mod routes;


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
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn options() {

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/auth", routes![auth::login])
        .mount("/user", routes![user::post_user])
        .mount("/", routes![options])
    
}