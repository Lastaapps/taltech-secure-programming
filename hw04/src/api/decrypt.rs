
use rocket::{http::CookieJar, response::Redirect};
use rocket_dyn_templates::{Template, context};

use crate::domain::roles::remove_jwt_token;

#[post("/decrypt-ceasar")]
pub async fn decrypt_ceasar_post() -> Template {
    Template::render("decrypted", context!{
        text: "Nope"
    })
}
