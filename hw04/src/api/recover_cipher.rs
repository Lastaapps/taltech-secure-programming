use rocket::{form::Form, http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};
use time::{macros::datetime, PrimitiveDateTime};

use crate::{
    domain::{database::BrutusDb, roles::Ceasar, DomainError},
    models::{GetCeasarForRecoveryDto, GetVigenereForRecoveryDto, UserUsernameDto},
};

use super::common::{format_date_for_web, now_primitive, CipherKindPayload};
use serde::Serialize;

#[derive(Serialize)]
struct CipherItem {
    username: String,
    id: i32,
    base64: String,
    created: String,
    updated: String,
}

#[get("/admin/recover-cipher")]
pub async fn recover_get(db: BrutusDb, user: Ceasar) -> Result<Template, DomainError> {
    let username = user.0;

    let deleted_ceasar = get_deleted_ciphers_ceasar(&db).await?;
    let deleted_vigenere = get_deleted_ciphers_vigener(&db).await?;

    Ok(Template::render(
        "recover",
        context! {
            username: username,
            ceasar_list: deleted_ceasar,
            vigenere_list: deleted_vigenere,
        },
    ))
}

async fn get_deleted_ciphers_ceasar(db: &BrutusDb) -> Result<Vec<CipherItem>, DomainError> {
    let data: Vec<(UserUsernameDto, GetCeasarForRecoveryDto)> = db
        .run(|conn| {
            use crate::schema;
            use diesel::prelude::*;
            use schema::ceasar::dsl as cdsl;
            use schema::users::dsl as udsl;

            schema::ceasar::table
                .inner_join(schema::users::table)
                .filter(cdsl::deleted.eq(true))
                .filter(udsl::deleted.eq(false))
                .order_by(cdsl::created)
                .select((
                    UserUsernameDto::as_select(),
                    GetCeasarForRecoveryDto::as_select(),
                ))
                .load::<(UserUsernameDto, GetCeasarForRecoveryDto)>(conn)
        })
        .await?;

    Ok(data
        .into_iter()
        .map(|(username, data)| CipherItem {
            id: data.id,
            username: username.username,
            base64: data.data,
            created: format_date_for_web(&data.created.assume_utc()),
            updated: format_date_for_web(&data.updated.assume_utc()),
        })
        .collect())
}

async fn get_deleted_ciphers_vigener(db: &BrutusDb) -> Result<Vec<CipherItem>, DomainError> {
    let data: Vec<(UserUsernameDto, GetVigenereForRecoveryDto)> = db
        .run(|conn| {
            use crate::schema;
            use diesel::prelude::*;
            use schema::users::dsl as udsl;
            use schema::vigenere::dsl as cdsl;

            schema::vigenere::table
                .inner_join(schema::users::table)
                .filter(cdsl::deleted.eq(true))
                .filter(udsl::deleted.eq(false))
                .order_by(cdsl::created)
                .select((
                    UserUsernameDto::as_select(),
                    GetVigenereForRecoveryDto::as_select(),
                ))
                .load::<(UserUsernameDto, GetVigenereForRecoveryDto)>(conn)
        })
        .await?;

    Ok(data
        .into_iter()
        .map(|(username, data)| CipherItem {
            id: data.id,
            username: username.username,
            base64: data.data,
            created: format_date_for_web(&data.created.assume_utc()),
            updated: format_date_for_web(&data.updated.assume_utc()),
        })
        .collect())
}

#[derive(FromForm)]
pub struct RecoverCipherPayload {
    id: i32,
    kind: CipherKindPayload,
}

#[post("/admin/recover-cipher", data = "<data>")]
pub async fn recover_post(
    db: BrutusDb,
    _user: Ceasar,
    data: Form<RecoverCipherPayload>,
) -> Result<Redirect, DomainError> {
    match data.kind {
        CipherKindPayload::Ceasar => {
            recover_cipher_ceasar(&db, data.id, now_primitive()).await?;
        }
        CipherKindPayload::Vigenere => {
            recover_cipher_vigenere(&db, data.id, now_primitive()).await?
        }
    };

    Ok(Redirect::to(uri!("/admin/recover-cipher")))
}

async fn recover_cipher_ceasar(
    db: &BrutusDb,
    cipher_id: i32,
    now: PrimitiveDateTime,
) -> Result<(), DomainError> {
    let rows = db
        .run(move |conn| {
            use crate::schema;
            use diesel::prelude::*;
            use schema::ceasar::dsl::*;

            diesel::update(ceasar)
                .filter(id.eq(cipher_id))
                .filter(deleted.eq(true))
                .set((deleted.eq(false), updated.eq(now)))
                .execute(conn)
        })
        .await?;

    if rows == 0 {
        Err(DomainError::CipherNotFound)?;
    }
    Ok(())
}

async fn recover_cipher_vigenere(
    db: &BrutusDb,
    cipher_id: i32,
    now: PrimitiveDateTime,
) -> Result<(), DomainError> {
    let rows = db
        .run(move |conn| {
            use crate::schema;
            use diesel::prelude::*;
            use schema::vigenere::dsl::*;

            diesel::update(vigenere)
                .filter(id.eq(cipher_id))
                .filter(deleted.eq(true))
                .set((deleted.eq(false), updated.eq(now)))
                .execute(conn)
        })
        .await?;

    if rows == 0 {
        Err(DomainError::CipherNotFound)?;
    }
    Ok(())
}
