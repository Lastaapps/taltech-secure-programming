use rocket::{
    form::{Form, FromForm},
    response::Redirect,
};

use crate::{domain::ciphers::encode_vigener, models::InsertVigenereDto, schema};
use crate::{
    domain::{ciphers::encode_ceasar, database::BrutusDb, roles::Antonius, DomainError},
    models::InsertCeasarDto,
};

use super::common::{decode_base64, get_user_id};

#[derive(FromForm)]
pub struct AddCipherCeasarPayload {
    #[field(validate = len(..1024))]
    data: String,
    is_base64: bool,
    shift: i32,
}

#[post("/add-cipher-ceasar", data = "<data>")]
pub async fn add_ceasar_post(
    db: BrutusDb,
    user: Antonius,
    data: Form<AddCipherCeasarPayload>,
) -> Result<Redirect, DomainError> {
    let username = user.0;

    let mut bytes = if data.is_base64 {
        decode_base64(&data.data)?
    } else {
        data.data.as_bytes().to_vec()
    };

    let encoded =
        encode_ceasar(&mut bytes, data.shift.into())?;
    let user_id = get_user_id(&db, username.as_str()).await?;

    let obj = InsertCeasarDto {
        user_id,
        data: encoded,
        shift: data.shift,
    };

    insert_ceasar(&db, obj).await?;

    Ok(Redirect::to(uri!("/")))
}

async fn insert_ceasar(db: &BrutusDb, obj: InsertCeasarDto) -> Result<(), DomainError> {
    db.run(|conn| {
        use diesel::prelude::*;
        use schema::ceasar::dsl::*;

        diesel::insert_into(ceasar).values(obj).execute(conn)
    })
    .await?;

    Ok(())
}

#[derive(FromForm)]
pub struct AddCipherVigenerPayload {
    #[field(validate = len(..1024))]
    data: String,
    is_base64: bool,
    #[field(validate = len(..1024))]
    key: String,
}

#[post("/add-cipher-vigener", data = "<data>")]
pub async fn add_vigener_post(
    db: BrutusDb,
    user: Antonius,
    data: Form<AddCipherVigenerPayload>,
) -> Result<Redirect, DomainError> {
    let username = user.0;

    let mut bytes = if data.is_base64 {
        decode_base64(&data.data)?
    } else {
        data.data.as_bytes().to_vec()
    };

    let key = decode_base64(&data.key)?;

    let (encoded, key) = encode_vigener(&mut bytes, key.as_slice())?;
    let user_id = get_user_id(&db, username.as_str()).await?;

    let obj = InsertVigenereDto {
        user_id,
        data: encoded,
        key,
    };

    insert_vigenere(&db, obj).await?;

    Ok(Redirect::to(uri!("/")))
}

async fn insert_vigenere(db: &BrutusDb, obj: InsertVigenereDto) -> Result<(), DomainError> {
    db.run(|conn| {
        use diesel::prelude::*;
        use schema::vigenere::dsl::*;

        diesel::insert_into(vigenere).values(obj).execute(conn)
    })
    .await?;

    Ok(())
}
