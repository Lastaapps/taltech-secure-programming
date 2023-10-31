use rocket::{Rocket, Build};
use rocket::{Request, fairing::AdHoc};

use rocket_dyn_templates::{Template, context};
use dotenv::dotenv;

mod database;
mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
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

#[get("/register")]
fn register_get() -> Template {
    Template::render("register", context! {})
}

#[post("/register")]
async fn register_post(db: BrutusDb) -> &'static str {
    // db.run(|c| ).await;

    "Hello, world!"
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

use rocket_sync_db_pools::{database, diesel};
#[database("brutus_db")]
struct BrutusDb(diesel::SqliteConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    // fix diesel rebuilding
    println!("cargo:rerun-if-changed=migrations");

    rocket::build()
        .mount("/", routes![index, login_get, login_post, register_get, register_post,])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .attach(BrutusDb::fairing())
        .attach(AdHoc::try_on_ignite("Database Migrations", migrate))
}

async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = BrutusDb::get_one(&rocket).await.expect("database connection");
    db.run(|conn| match conn.run_pending_migrations(MIGRATIONS) {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
    .await
}

