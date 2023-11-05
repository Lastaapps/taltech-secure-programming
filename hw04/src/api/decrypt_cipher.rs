use rocket::{http::CookieJar, response::Redirect, form::Form};
use rocket_dyn_templates::{context, Template};

use crate::domain::roles::Antonius;

use super::common::CipherKindPayload;

#[derive(FromForm)]
pub struct DecryptCeasarPayload {
    id: i64,
    kind: CipherKindPayload,
    is_base64: bool,
}

#[post("/decrypt-ceasar", data = "<data>")]
pub async fn decrypt_cipher_post(user: Antonius, data: Form<DecryptCeasarPayload>) -> Template {
    Template::render(
        "decrypted",
        context! {
            text: "Nope"
        },
    )
}
