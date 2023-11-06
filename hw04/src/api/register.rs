use crate::domain::database::BrutusDb;
use crate::domain::jwt::create_token;
use crate::domain::roles::{store_jwt_token, KickFromLogin};
use crate::domain::Outcome;
use crate::models::CreateUserDto;
use crate::util::username_validator;
use crate::{domain::hashing, schema};
use diesel::prelude::*;
use either::Either;

use rocket::http::CookieJar;
use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{context, Template};

#[get("/register")]
pub async fn register_get(_kick: KickFromLogin) -> Template {
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
    _kick: KickFromLogin,
    data: Form<RegisterForm>,
) -> Outcome<Either<Template, Redirect>> {
    println!("Registering new user {}", &data.username);
    let loc_username = data.username.clone();

    let username_cnt = db
        .run(move |conn| {
            use crate::schema::users::dsl::*;

            users
                .filter(username.eq(loc_username.as_str()))
                .limit(1)
                .count()
                .get_result::<i64>(conn)
        })
        .await?;
    if username_cnt != 0 {
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
