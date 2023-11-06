use rocket::{form::Form, response::Redirect};
use time::PrimitiveDateTime;

use crate::domain::{database::BrutusDb, roles::Antonius, DomainError};

use super::common::{get_user_id, now_primitive, CipherKindPayload};

#[derive(FromForm)]
pub struct DeleteCipherPayload {
    id: i32,
    kind: CipherKindPayload,
}

#[post("/delete-cipher", data = "<data>")]
pub async fn delete_cipher_post(
    db: BrutusDb,
    user: Antonius,
    data: Form<DeleteCipherPayload>,
) -> Result<Redirect, DomainError> {
    let username = user.0;

    let user_id = get_user_id(&db, &username).await?;
    let now = now_primitive();
    match data.kind {
        CipherKindPayload::Ceasar => delete_cipher_ceasar(&db, user_id, data.id, now).await?,
        CipherKindPayload::Vigenere => delete_cipher_vigenere(&db, user_id, data.id, now).await?,
    };

    Ok(Redirect::to(uri!("/")))
}

async fn delete_cipher_ceasar(
    db: &BrutusDb,
    user_id: i32,
    cipher_id: i32,
    now: PrimitiveDateTime,
) -> Result<(), DomainError> {
    let loc_user_id = user_id;
    let loc_cipher_id = cipher_id;

    let rows = db
        .run(move |conn| {
            use crate::schema::ceasar::dsl::*;
            use diesel::prelude::*;

            diesel::update(ceasar)
                .filter(user_id.eq(loc_user_id))
                .filter(id.eq(loc_cipher_id))
                .filter(deleted.eq(false))
                .set((deleted.eq(true), updated.eq(now)))
                .execute(conn)
        })
        .await?;
    if rows == 0 {
        Err(DomainError::CipherNotFound)?;
    }
    Ok(())
}

async fn delete_cipher_vigenere(
    db: &BrutusDb,
    user_id: i32,
    cipher_id: i32,
    now: PrimitiveDateTime,
) -> Result<(), DomainError> {
    let loc_user_id = user_id;
    let loc_cipher_id = cipher_id;

    let rows = db
        .run(move |conn| {
            use crate::schema::vigenere::dsl::*;
            use diesel::prelude::*;

            diesel::update(vigenere)
                .filter(user_id.eq(loc_user_id))
                .filter(id.eq(loc_cipher_id))
                .filter(deleted.eq(false))
                .set((deleted.eq(true), updated.eq(now)))
                .execute(conn)
        })
        .await?;
    if rows == 0 {
        Err(DomainError::CipherNotFound)?;
    }
    Ok(())
}
