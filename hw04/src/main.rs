use rocket::Request;
use rocket::response::Redirect;

use rocket_dyn_templates::{Template, tera::Tera, context};

mod database;

#[macro_use]
extern crate rocket;

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

#[get("/register")]
fn register_get() -> Template {
    Template::render("register", context! {})
}

#[post("/register")]
fn register_post() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, login_get, login_post, register_get, register_post,])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}

