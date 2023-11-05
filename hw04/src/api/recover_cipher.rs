use rocket::{http::CookieJar, response::Redirect, form::Form};
use rocket_dyn_templates::{Template, context};
use time::macros::datetime;

use crate::domain::roles::Ceasar;

use super::common::{CipherKindPayload, format_date_for_web};
use serde::Serialize;

#[derive(Serialize)]
struct CipherItem {
    username: String,
    id: i64,
    base64: String,
    created: String,
    updated: String,
}

#[get("/admin/recover-cipher")]
pub async fn recover_get(user: Ceasar) -> Template {
    let username = user.0;

    Template::render(
        "recover",
        context! {
            username: username,
            ceasar_list: vec![CipherItem{
                username: "Jára Cimrman".into(), 
                id: 1,
                base64: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
                created: format_date_for_web(&datetime!(2021-01-01 0:00:00 +0:00)),
                updated: format_date_for_web(&datetime!(2023-02-02 2:02:00 +0:00)),
            }]
        },
    )
}

#[derive(FromForm)]
pub struct RecoverCipherPayload {
    id: i64,
    kind: CipherKindPayload,
}

#[post("/admin/recover-cipher", data = "<data>")]
pub async fn recover_post(user: Ceasar, data: Form<RecoverCipherPayload>) -> Redirect {
    Redirect::to(uri!("/login"))
}
