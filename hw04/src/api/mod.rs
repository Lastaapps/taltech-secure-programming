mod add_cipher;
mod common;
mod decrypt_cipher;
mod delete_cipher;
mod index;
mod login;
mod logout;
mod recover_cipher;
mod register;

use rocket::{response::Redirect, Build, Request, Rocket};
use rocket_dyn_templates::{context, Template};
use std::path::Path;
use rocket::fs::{NamedFile, relative};


#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[catch(401)]
fn unauthorized(req: &Request<'_>) -> Redirect {
    let path = req.uri().path().as_str();

    eprintln!("Catching 401, returnUrl is: {}", path);

    Redirect::to(uri!(crate::api::login::login_get(return_url = Some(path))))
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
    let path = Path::new(relative!("static/favicon.ico"));
    NamedFile::open(path).await.ok()
}

#[get("/styles.css")]
pub async fn styles() -> Option<NamedFile> {
    let path = Path::new(relative!("static/styles.css"));
    NamedFile::open(path).await.ok()
}

pub struct FuckRustApi(Rocket<Build>);
impl FuckRustApi {
    pub fn mount_api(self) -> Rocket<Build> {
        self.0
            .mount(
                "/",
                routes![
                    //others
                    favicon,
                    styles,
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
            .register("/", catchers![not_found, unauthorized,])
    }
}

impl From<Rocket<Build>> for FuckRustApi {
    fn from(value: Rocket<Build>) -> Self {
        FuckRustApi(value)
    }
}
