pub mod login;
pub mod logout;
pub mod register;

use rocket::{Build, Rocket, Request};
use rocket_dyn_templates::{Template, context};

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

pub struct FuckRustApi(Rocket<Build>);
impl FuckRustApi {
    pub fn mount_api(self) -> Rocket<Build> {
        self.0
            .mount(
                "/",
                routes![
                    index,
                    // register
                    crate::api::register::register_get,
                    crate::api::register::register_post,
                    // login
                    crate::api::login::login_get,
                    crate::api::login::login_post,
                    // logout
                    crate::api::logout::logout_get,
                    crate::api::logout::logout_post,
                ],
            )
            .register("/", catchers![not_found])
    }
}

impl From<Rocket<Build>> for FuckRustApi {
    fn from(value: Rocket<Build>) -> Self {
        FuckRustApi(value)
    }
}
