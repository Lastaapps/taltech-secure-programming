
pub mod ciphers;
pub mod database;
pub mod jwt;
pub mod roles;
pub mod hashing;

use rocket::{response::Responder, http::Status};


pub type Outcome<T> = Result<T, DomainError>;

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum DomainError {
    General(String),
    Diesel(diesel::result::Error),
    Argon2(argon2::password_hash::Error),
    JWT(jsonwebtoken::errors::Error),

    NotBase64,
    CipherNotFound,
}

impl From<diesel::result::Error> for DomainError {
    fn from(value: diesel::result::Error) -> Self {
        DomainError::Diesel(value)
    }
}

impl From<argon2::password_hash::Error> for DomainError {
    fn from(value: argon2::password_hash::Error) -> Self {
        DomainError::Argon2(value)
    }
}

impl From<jsonwebtoken::errors::Error> for DomainError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        DomainError::JWT(value)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for DomainError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            DomainError::General(e) => {
                eprint!("{}", e);
                Err(Status::InternalServerError)
            },
            DomainError::Diesel(e) => {
                eprint!("{}", e);
                Err(Status::InternalServerError)
            },
            DomainError::Argon2(e) => {
                eprint!("{}", e);
                Err(Status::InternalServerError)
            },
            DomainError::JWT(e) => {
                eprint!("{}", e);
                Err(Status::Unauthorized)
            },
            DomainError::NotBase64 => {
                eprint!("Not a base64 payload");
                Err(Status::BadRequest)
            },
            DomainError::CipherNotFound => {
                eprint!("Cipher not found");
                Err(Status::NotFound)
            },
        }
    }
}

