use either::Either;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use time::{format_description, macros::datetime, Date, OffsetDateTime, PrimitiveDateTime, Time};

use crate::domain::roles::{Antonius, Ceasar};
use serde::Serialize;
use super::common::format_date_for_web;

#[derive(Serialize)]
struct CeasarCipher {
    id: i64,
    base64: String,
    shift: i64,
    created: String,
    updated: String,
}

#[derive(Serialize)]
struct VigenerCipher {
    id: i64,
    base64: String,
    key: String,
    created: String,
    updated: String,
}

#[get("/")]
pub async fn index_get(
    user: Option<Antonius>,
    admin: Option<Ceasar>,
) -> Either<Template, Redirect> {
    let username = match user {
        Some(user) => user.0,
        None => return Either::Right(Redirect::to("/login")),
    };

    return Either::Left(Template::render(
        "index",
        context! {
            username: username,
            admin: admin.is_some(),
            ceasar_list: vec![CeasarCipher{
                id: 0,
                base64: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaamno".into(),
                shift: 42,
                created: format_date_for_web(&datetime!(2021-01-01 0:00:00 +0:00)),
                updated: format_date_for_web(&datetime!(2023-02-02 2:02:00 +0:00)),
            }],
        },
    ));
}

