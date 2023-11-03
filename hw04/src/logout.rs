use rocket::{http::CookieJar, response::Redirect};
use rocket_dyn_templates::{Template, context};

use crate::roles::remove_jwt_token;

#[get("/logout")]
pub async fn logout_get() -> Template {
    Template::render(
        "logout",
        context! {
            error_msg: "",
        },
    )
}

#[post("/logout")]
pub async fn logout_post(cookies: &CookieJar<'_>) -> Redirect {
    remove_jwt_token(cookies);
    Redirect::to(uri!("/login"))
}
