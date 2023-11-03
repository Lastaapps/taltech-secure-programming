use crate::jwt::create_token;
use crate::roles::store_jwt_token;
use crate::{database::BrutusDb, roles::Antonius};
use crate::domain::Outcome;
use crate::models::LoginUserDto;
use crate::security;
use crate::util::username_validator;
use diesel::prelude::*;
use either::Either;

use rocket::{form::Form, response::Redirect, http::CookieJar};
use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub async fn login_get() -> Template {
    Template::render(
        "login",
        context! {
            error_msg: "",
        },
    )
}

#[derive(FromForm)]
pub struct LoginForm {
    #[field(validate = len(1..))]
    #[field(validate = username_validator())]
    username: String,
    #[field(validate = len(8..))]
    password: String,
}

#[post("/login", data = "<data>")]
pub async fn login_post(
    db: BrutusDb,
    cookies: &CookieJar<'_>,
    user: Option<Antonius>,
    data: Form<LoginForm>,
) -> Outcome<Either<Template, Redirect>> {
    // already logged in
    if user.is_some() {
        return Ok(Either::Right(Redirect::to(uri!("/"))));
    }

    println!("Login user {}", &data.username);
    let username_copy = data.username.clone();
    let err_msg = "Username does not exist or the password is incorrect.";

    let user = match db
        .run(move |c| {
            use crate::schema::users::dsl::*;

            users
                .filter(username.eq(username_copy.as_str()))
                .limit(1)
                .select(LoginUserDto::as_select())
                .first(c)
                .optional()
        })
        .await?
    {
        Some(user) => user,
        None => {
            let page = Template::render(
                "login",
                context! {
                    error_msg: err_msg,
                },
            );
            return Ok(Either::Left(page));
        }
    };

    println!("Checking password");
    if !security::verify_password(&data.password, &user.password_hash)? {
        let page = Template::render(
            "login",
            context! {
                error_msg: err_msg,
            },
        );
        return Ok(Either::Left(page));
    }
    println!("User password matched");

    if user.deleted {
        let page = Template::render(
            "login",
            context! {
                error_msg: "User account is deactivated.",
            },
        );
        return Ok(Either::Left(page));
    }

    println!("Creating JWT token");
    let token = create_token(&data.username)?;
    store_jwt_token(cookies, &token);

    println!("User logged in");
    Ok(Either::Right(Redirect::to(uri!("/"))))
}
