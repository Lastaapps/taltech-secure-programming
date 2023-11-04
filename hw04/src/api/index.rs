use either::Either;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

use crate::domain::roles::{Antonius, Ceasar};
use serde::Serialize;

#[derive(Serialize)]
struct CeasarCipher {
    id: i64,
    base64: String,
    shift: i64,
}

#[derive(Serialize)]
struct VigenerCipher {
    id: i64,
    base64: String,
    key: String,
}

#[get("/")]
pub async fn index_get(
    user: Option<Antonius>,
    admin: Option<Ceasar>,
) -> Either<Template, Redirect> {
    // let user = match user {
    //     Some(user) => user,
    //     None => return Either::Right(Redirect::to("/login")),
    // };

    // match admin {
    //     Some(admin) => render_admin(admin),
    //     None => render_user(user),
    // };

    return Either::Left(Template::render(
        "index",
        context! {
            username: "Jára Cimrman",
            admin: true,
            ceasar_list: vec![CeasarCipher{id: 0, base64: "mno".into(), shift: 42}],
        },
    ));
}

fn render_admin(admin: Ceasar) {
    let username: String = admin.0;
}

fn render_user(user: Antonius) {
    let username: String = user.0;
}
