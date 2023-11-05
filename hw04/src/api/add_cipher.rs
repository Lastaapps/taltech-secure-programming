use rocket::{
    form::{Form, FromForm},
    response::Redirect,
};

use crate::schema;
use crate::{
    domain::{ciphers::encode_ceasar, database::BrutusDb, roles::Antonius, DomainError},
    models::InsertCeasarDto,
};

use super::common::{get_user_id, decode_base64};

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
        encode_ceasar(&mut bytes, data.shift.into()).map_err(|e| DomainError::General(e))?;
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
pub async fn add_vigener_post(user: Antonius, data: Form<AddCipherVigenerPayload>) -> Redirect {
    Redirect::to(uri!("/login"))
}
