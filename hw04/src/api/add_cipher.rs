
use rocket::{form::{Form, FromForm}, http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};

use crate::domain::roles::Antonius;

#[derive(FromForm)]
pub struct AddCipherCeasarPayload {
    #[field(validate = len(..1024))]
    data: String,
    is_base64: bool,
    shift: i64,
}

#[post("/add-cipher-ceasar", data = "<data>")]
pub async fn add_ceasar_post(user: Antonius, data: Form<AddCipherCeasarPayload>) -> Redirect {
    Redirect::to(uri!("/login"))
}

#[derive(FromForm)]
pub struct AddCipherVigenerPayload {
    #[field(validate = len(..1024))]
    data: String,
    is_base64: bool,
    #[field(validate = len(..1024))]
    key: String,
}

#[post("/add-cipher-vigener", data = "<data>")]
pub async fn add_vigener_post(user: Antonius, data: Form<AddCipherVigenerPayload>) -> Redirect {
    Redirect::to(uri!("/login"))
}
