use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket_dyn_templates::Template;

mod api;
mod domain;

mod models;
mod schema;
mod util;


#[macro_use]
extern crate lazy_static;

use crate::domain::database::BrutusDb;
use crate::api::FuckRustApi;

#[macro_use]
extern crate rocket;
extern crate diesel_migrations;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    // fix diesel rebuilding
    println!("cargo:rerun-if-changed=migrations");

    FuckRustApi::from(rocket::build())
        .mount_api()
        .attach(Template::fairing())
        .attach(BrutusDb::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            domain::database::migrate,
        ))
}
