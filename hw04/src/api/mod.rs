
mod add_cipher;
mod common;
mod decrypt_cipher;
mod delete_cipher;
mod index;
mod login;
mod logout;
mod recover_cipher;
mod register;

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

pub struct FuckRustApi(Rocket<Build>);
impl FuckRustApi {
    pub fn mount_api(self) -> Rocket<Build> {
        self.0
            .mount(
                "/",
                routes![
                    // register
                    crate::api::register::register_get,
                    crate::api::register::register_post,
                    // login
                    crate::api::login::login_get,
                    crate::api::login::login_post,
                    // logout
                    crate::api::logout::logout_get,
                    crate::api::logout::logout_post,
                    // index
                    crate::api::index::index_get,
                    // ciphers
                    crate::api::add_cipher::add_ceasar_post,
                    crate::api::add_cipher::add_vigener_post,
                    crate::api::decrypt_cipher::decrypt_cipher_post,
                    crate::api::delete_cipher::delete_cipher_post,
                    // recover
                    crate::api::recover_cipher::recover_get,
                    crate::api::recover_cipher::recover_post,
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
