
extern crate bcrypt;

use bcrypt::verify;
use chrono::Utc;
use errors::Error;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub mod errors;

const JWT_SECRET: &[u8] = b"superdupergeheim";

pub fn authorize(username: &String, password: &String) -> Result<String, Error> {

    let user_repository = datalayer::user::UserRepository {};

    let user= user_repository.get_user_by_username(username)
        .map_err(|_| Error::WrongCredentialsError)?;

    let result = verify(password, user.password.unwrap().as_str()).unwrap();

    if !result {
        return Err(Error::WrongCredentialsError);
    }

    create_token(user.user_id, user.username.unwrap())

}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    user_id: String,
    username: String,
    exp: usize
}

fn create_token(user_id: String, username: String) -> Result<String, Error> {

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(300))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        user_id: user_id.to_owned(),
        username: username,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    return encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| Error::JWTTokenCreationError)
    
}

pub fn validate_token(token: &str) -> Result<Claims, Error> {

    let access_token = &token[7..];

    println!("{}", access_token);

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