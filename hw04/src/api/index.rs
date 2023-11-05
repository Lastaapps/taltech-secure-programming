use either::Either;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use time::{format_description, macros::datetime, Date, OffsetDateTime, PrimitiveDateTime, Time};

use super::common::{format_date_for_web, get_user_id};
use crate::{
    domain::{
        database::BrutusDb,
        roles::{Antonius, Ceasar},
        DomainError,
    },
    models::GetCeasarDto,
};
use serde::Serialize;

#[derive(Serialize)]
struct CeasarCipher {
    id: i32,
    base64: String,
    shift: i32,
    created: String,
    updated: String,
}

#[derive(Serialize)]
struct VigenerCipher {
    id: i32,
    base64: String,
    key: String,
    created: String,
    updated: String,
}

#[get("/")]
pub async fn index_get(
    db: BrutusDb,
    user: Option<Antonius>,
    admin: Option<Ceasar>,
) -> Result<Either<Template, Redirect>, DomainError> {
    let username = match user {
        Some(user) => user.0,
        None => return Ok(Either::Right(Redirect::to("/login"))),
    };

    let user_id = get_user_id(&db, &username).await?;
    let ceasar_list = get_ceasars(&db, user_id).await;

    Ok(Either::Left(Template::render(
        "index",
        context! {
            username: username,
            admin: admin.is_some(),
            ceasar_list: ceasar_list,
        },
    )))
}

async fn get_ceasars(db: &BrutusDb, user_id: i32) -> Vec<CeasarCipher> {
    let loc_user_id = user_id;
    let data: Vec<GetCeasarDto> = db
        .run(move |conn| {
            use crate::schema::ceasar::dsl::*;
            use diesel::prelude::*;

            ceasar
                .filter(user_id.eq(loc_user_id))
                .filter(deleted.eq(false))
                .order_by(created)
                .select(GetCeasarDto::as_select())
                .load(conn)
        })
        .await
        .unwrap();

    data.into_iter()
        .map(|cipher| CeasarCipher {
            id: cipher.id,
            base64: cipher.data,
            shift: cipher.shift,
            created: format_date_for_web(&cipher.created.assume_utc()),
            updated: format_date_for_web(&cipher.created.assume_utc()),
        })
        .collect()
}
