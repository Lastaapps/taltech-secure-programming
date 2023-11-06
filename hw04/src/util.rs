use regex::Regex;
use rocket::form::{self, Error};

pub fn username_validator<'v>(username: &str) -> form::Result<'v, ()> {
    let re = Regex::new(r"^([0-9]|[a-z]|[A-Z])+$").unwrap();
    match re.captures(username) {
        Some(_) => Ok(()),
        None => Err(Error::validation("Username contains invalid characters"))?,
    }
}
