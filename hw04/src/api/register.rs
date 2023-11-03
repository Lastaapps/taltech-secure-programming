use crate::domain::database::BrutusDb;
use crate::domain::Outcome;
use crate::domain::jwt::create_token;
use crate::models::{CreateUserDto, UsersCheckDto};
use crate::domain::roles::{Antonius, store_jwt_token};
use crate::util::username_validator;
use crate::{schema, domain::hashing};
use diesel::prelude::*;
use either::Either;

use rocket::http::CookieJar;
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{context, Template};

#[get("/register")]
pub async fn register_get() -> Template {
    Template::render(
        "register",
        context! {
            error_msg: "",
        },
    )
}

#[derive(FromForm)]
pub struct RegisterForm {
    #[field(validate = len(1..))]
    #[field(validate = username_validator())]
    username: String,
    #[field(validate = len(8..))]
    password: String,
}

#[post("/register", data = "<data>")]
pub async fn register_post(
    db: BrutusDb,
    cookies: &CookieJar<'_>,
    user: Option<Antonius>,
    data: Form<RegisterForm>,
) -> Outcome<Either<Template, Redirect>> {
    // already logged in
    if user.is_some() {
        return Ok(Either::Right(Redirect::to(uri!("/"))))
    }


    println!("Registering new user {}", &data.username);
    let loc_username = data.username.clone();

    if db
        .run(move |c| {
            use crate::schema::users::dsl::*;

            users
                .filter(username.eq(loc_username.as_str()))
                .limit(1)
                .select(UsersCheckDto::as_select())
                .first(c)
                .optional()
        })
        .await?
        .is_some()
    {
        let page = Template::render(
            "register",
            context! {
                error_msg: "Username already existst",
            },
        );
        return Ok(Either::Left(page));
    }

    println!("Hashing password");
    let hashed = hashing::hash_password(&data.password)?;

    let obj = CreateUserDto {
        username: data.username.clone(),
        password_hash: hashed,
    };
    let username = obj.username.clone();

    // There is a race condition with the username, TODO resolve better
    println!("Storing to db");
    db.run(|c| {
        diesel::insert_into(schema::users::table)
            .values(obj)
            .execute(c)
    })
    .await?;

    println!("Creating JWT token");
    let token = create_token(&username)?;
    store_jwt_token(cookies, &token);

    println!("User created");

    Ok(Either::Right(Redirect::to(uri!("/"))))
}
