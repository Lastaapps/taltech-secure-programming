use crate::database::BrutusDb;
use crate::domain::{DomainError, Outcome};
use crate::models::{CreateUserDto, UsersCheckDto};
use crate::{schema, security};
use diesel::prelude::*;
use either::Either;
use crate::util::username_validator;
use rocket::response::Responder;
use rocket::Response;
use rocket::{
    form::{validate::len, Form, Strict},
    futures::stream::Select,
    response::{Debug, Redirect},
};
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
    data: Form<RegisterForm>,
) -> Outcome<Either<Template, Redirect>> {
    use crate::schema::users::dsl::*;

    println!("Registering new user {}", &data.username);
    let loc_username = data.username.clone();

    if let Some(_) = db
        .run(move |c| {
            users
                .filter(username.eq(loc_username.as_str()))
                .limit(1)
                .select(UsersCheckDto::as_select())
                .first(c)
                .optional()
        })
        .await?
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
    let hashed = security::hash_password(&data.password)?;

    let obj = CreateUserDto {
        username: data.username.clone(),
        password_hash: hashed,
    };

    // There is a race condition with the username, TODO resolve better
    println!("Storing to db");
    db.run(|c| {
        diesel::insert_into(schema::users::table)
            .values(obj)
            .execute(c)
    })
    .await?;

    println!("User created");
    Ok(Either::Right(Redirect::temporary(uri!("/login"))))
}
