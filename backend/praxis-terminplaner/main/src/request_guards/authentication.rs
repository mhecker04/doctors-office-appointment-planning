use crate::response_models::{NetworkResponse, Response, ResponseBody};
use authentication::{validate_token, Claims};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use session::{Session, SESSION_STORAGE};

pub struct Token {
    #[allow(dead_code)]
    claims: Claims,
}

impl Token {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        let error_response = Response {
            body: ResponseBody::Message(String::from(
                "Error validating JWT token - No token provided",
            )),
        };

        let error = Outcome::Error((
            Status::Unauthorized,
            NetworkResponse::Unauthorized(serde_json::to_string(&error_response).unwrap()),
        ));

        match req.headers().get_one("Authorization") {
            None => error,
            Some(key) => match validate_token(key) {
                Ok(claims) => {
                    SESSION_STORAGE.lock().await.register(Session {
                        user_id: claims.user_id.clone(),
                        username: claims.username.clone(),
                    });
                    Outcome::Success(Token { claims })
                }
                Err(_) => error,
            },
        }
    }
}
