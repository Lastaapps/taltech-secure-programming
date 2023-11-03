use rocket::{response::Responder, http::Status};


pub type Outcome<T> = Result<T, DomainError>;

#[derive(Debug)]
pub enum DomainError {
    Diesel(diesel::result::Error),
    Argon2(argon2::password_hash::Error),
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

impl<'r, 'o: 'r> Responder<'r, 'o> for DomainError {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self {
            DomainError::Diesel(e) => {
                eprint!("{}", e);
                return Err(Status::InternalServerError)
            },
            DomainError::Argon2(e) => {
                eprint!("{}", e);
                return Err(Status::InternalServerError)
            },
        }
    }
}

