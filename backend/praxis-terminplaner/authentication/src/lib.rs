
extern crate bcrypt;

use chrono::Utc;
use errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub mod errors;

const JWT_SECRET: &[u8] = b"superdupergeheim";

pub async fn authorize(username: &String, password: &String) -> Result<String, Error> {

    let user_repository = datalayer::user::UserRepository {};

    let user = user_repository.get_by_username(username)
        .await
        .map_err(|_| Error::WrongCredentialsError)?;

    let result = bcrypt::verify(password, user.password.as_str());

    match result {
        Err(_) => Err(Error::WrongCredentialsError),
        Ok(_) => create_token(user.user_id.unwrap(), user.username)
    }
    
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    exp: usize
}

fn create_token(user_id: String, username: String) -> Result<String, Error> {

    let expiration = Utc::now()
        // todo change later just for testing purposes set so high
        .checked_add_signed(chrono::Duration::seconds(30000))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        user_id: user_id.to_owned(),
        username: username,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
    
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {

    let access_token = &token[7..];

    let decode = 
        decode::<Claims>(&access_token, 
            &DecodingKey::from_secret(JWT_SECRET), 
            &Validation::new(Algorithm::HS512))
            .map_err(|_| Error::JWTTokenError);

    match decode {
        Ok(v) => Ok(v.claims),
        Err(e) => Err(e) 
    }

}

pub fn hash(password: &str) -> String {

    let hash = bcrypt::hash(password, 12);

    hash.unwrap()

}
