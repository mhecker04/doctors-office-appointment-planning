
#[derive(Debug)]
pub enum Error {

    WrongCredentialsError,
    JWTTokenError,
    JWTTokenCreationError,
    NoAuthHeaderError,
    InvalidAuthHeaderError,
    NoPermissionError,
}
