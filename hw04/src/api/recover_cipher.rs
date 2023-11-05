use rocket::{http::CookieJar, response::Redirect, form::Form};
use rocket_dyn_templates::{Template, context};

use crate::domain::roles::Ceasar;

use super::common::CipherKindPayload;

#[get("/recover")]
pub async fn recover_get(user: Ceasar) -> Template {
    Template::render(
        "recover",
        context! {
            error_msg: "",
        },
    )
}

#[derive(FromForm)]
pub struct RecoverCipherPayload {
    id: i64,
    kind: CipherKindPayload,
}

#[post("/recover-cipher", data = "<data>")]
pub async fn recover_post(user: Ceasar, data: Form<RecoverCipherPayload>) -> Redirect {
    Redirect::to(uri!("/login"))
}
