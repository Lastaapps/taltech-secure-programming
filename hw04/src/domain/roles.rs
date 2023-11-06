use diesel::prelude::*;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Outcome, Request};

use crate::domain::database::BrutusDb;
use crate::{domain::jwt::verify_token, domain::DomainError};

pub struct Antonius(pub String); // user
pub struct Ceasar(pub String); // admin
pub struct KickFromLogin();

static JWT_COOKIE_KEY: &str = "jwt";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Antonius {
    type Error = DomainError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let token = match req.cookies().get_private(JWT_COOKIE_KEY) {
            Some(token) => token,
            None => {
                eprintln!("JWT token missing");
                return Outcome::Error((
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
                return Outcome::Error((Status::Unauthorized, e));
            }
        };

        let db = req.guard::<BrutusDb>().await.unwrap();
        let non_deleted_cnt = db
            .run(|conn| {
                let local_username = &username;
                use crate::schema::users::dsl::*;
                users
                    .filter(username.eq(local_username))
                    .filter(deleted.eq(false))
                    .limit(1)
                    .count()
                    .get_result::<i64>(conn)
            })
            .await
            .unwrap();
        if non_deleted_cnt == 0 {
            eprintln!("User deleted");
            return Outcome::Error((
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

        let with_admin_role_cnt = db
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
            .unwrap();
        if with_admin_role_cnt == 0 {
            eprintln!("Invalid admin access attempt");
            return Outcome::Error((
                Status::Forbidden,
                DomainError::General("Invalid admin access attempt".into()),
            ));
        }

        Outcome::Success(Ceasar(username))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for KickFromLogin {
    type Error = DomainError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let status = req.guard::<Option<Antonius>>().await.unwrap();

        if status.is_some() {
            Outcome::Error((
                Status::PreconditionFailed,
                DomainError::General("Already logged in".into()),
            ))
        } else {
            Outcome::Success(KickFromLogin())
        }
    }
}

pub fn store_jwt_token(cookies: &CookieJar, token: &str) {
    cookies.add_private(Cookie::new(JWT_COOKIE_KEY.to_string(), token.to_string()))
}

pub fn remove_jwt_token(cookies: &CookieJar) {
    cookies.remove_private(Cookie::from(JWT_COOKIE_KEY))
}
