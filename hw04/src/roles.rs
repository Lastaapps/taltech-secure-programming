use diesel::prelude::*;
use rocket::http::{Status, Cookie, CookieJar};
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Outcome, Request};

use crate::database::BrutusDb;
use crate::models::UsersCheckDto;
use crate::{domain::DomainError, jwt::verify_token};

pub struct Antonius(String); // user
pub struct Ceasar(String); // admin

static JWT_COOKIE_KEY: &str = "jwt";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Antonius {
    type Error = DomainError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match req.cookies().get(JWT_COOKIE_KEY) {
            Some(token) => token,
            None => {
                eprintln!("JWT token missing");
                return Outcome::Failure((
                    Status::Unauthorized,
                    DomainError::General("JWT token missing".into()),
                ));
            }
        };
        let token = token.value();
        let username = match verify_token(token) {
            Ok(username) => username,
            Err(e) => {
                eprintln!("Token validation failed: {:?}", e);
                return Outcome::Failure((Status::BadRequest, e));
            }
        };

        let db = req.guard::<BrutusDb>().await.unwrap();
        if db
            .run(|conn| {
                let local_username = &username;
                use crate::schema::users::dsl::*;
                users
                    .filter(username.eq(local_username))
                    .filter(deleted.eq(false))
                    .limit(1)
                    .select(UsersCheckDto::as_select())
                    .first(conn)
                    .optional()
            })
            .await
            .unwrap()
            .is_none()
        {
            eprintln!("User deleted");
            return Outcome::Failure((
                Status::Forbidden,
                DomainError::General("User deleted".into()),
            ));
        };

        Outcome::Success(Antonius(username))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Ceasar {
    type Error = DomainError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = req.guard::<BrutusDb>().await.unwrap();
        let username = try_outcome!(req.guard::<Antonius>().await).0;
        let username_copy = username.clone();

        if db
            .run(|conn| {
                use crate::schema;

                schema::roles_users::table
                    .inner_join(schema::users::table)
                    .inner_join(schema::roles::table)
                    .filter(schema::users::dsl::username.eq(username_copy))
                    .filter(schema::roles::dsl::name.eq("admin"))
                    .limit(1)
                    .count()
                    .get_result::<i64>(conn)
            })
            .await
            .unwrap()
            == 0
        {
            eprintln!("Invalid admin access attempt");
            return Outcome::Failure((
                Status::Forbidden,
                DomainError::General("Invalid admin access attempt".into()),
            ));
        }

        Outcome::Success(Ceasar(username))
    }
}

pub fn store_jwt_token(cookies: &CookieJar, token: &str) {
    cookies.add_private(Cookie::new(JWT_COOKIE_KEY.to_string(), token.to_string()))
}

pub fn remove_jwt_token(cookies: &CookieJar) {
    cookies.remove_private(Cookie::named(JWT_COOKIE_KEY))
}

