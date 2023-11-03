use dotenv::dotenv;
use rocket::{fairing::AdHoc, Request};
use rocket_dyn_templates::{context, Template};

mod database;
mod domain;
mod models;
mod register;
mod schema;
mod security;

use crate::database::BrutusDb;

#[macro_use]
extern crate rocket;
extern crate diesel_migrations;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/login")]
fn login_get() -> Template {
    Template::render("login", context! {})
}

#[post("/login")]
fn login_post() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    // fix diesel rebuilding
    println!("cargo:rerun-if-changed=migrations");

    rocket::build()
        .mount("/", routes![index,])
        .mount("/", routes![login_get, login_post])
        .mount(
            "/",
            routes![
                crate::register::register_get,
                crate::register::register_post,
            ],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .attach(BrutusDb::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            database::migrate,
        ))
}
