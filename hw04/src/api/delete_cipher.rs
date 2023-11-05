use rocket::{http::CookieJar, response::Redirect, form::Form};
use rocket_dyn_templates::{Template, context};

use crate::domain::roles::Antonius;

use super::common::CipherKindPayload;

#[derive(FromForm)]
pub struct DeleteCipherPayload {
    id: i64,
    kind: CipherKindPayload,
}

#[post("/delete-cipher", data = "<data>")]
pub async fn delete_cipher_post(user: Antonius, data: Form<DeleteCipherPayload>) -> Redirect {
    Redirect::to(uri!("/login"))
}
