use base64::{engine::general_purpose, Engine};
use rocket::{form::Form, http::CookieJar, response::Redirect};
use rocket_dyn_templates::{context, Template};

use crate::{
    domain::{database::BrutusDb, roles::Antonius, DomainError, ciphers::decode_ceasar},
    models::GetCeasarInternalsDto,
    schema::ceasar,
};

use super::common::{get_user_id, CipherKindPayload};

#[derive(FromForm)]
pub struct DecryptCeasarPayload {
    id: i32,
    kind: CipherKindPayload,
    is_base64: bool,
}

#[post("/decrypt-cipher", data = "<data>")]
pub async fn decrypt_cipher_post(
    db: BrutusDb,
    user: Antonius,
    data: Form<DecryptCeasarPayload>,
) -> Result<Template, DomainError> {
    let username = user.0;

    let user_id = get_user_id(&db, &username).await?;
    let res: String = match data.kind {
        CipherKindPayload::Ceasar => {
            let cipher = get_cipher_id(&db, user_id, data.id).await?;
            let bytes = decode_ceasar(&cipher.data, cipher.shift.into())
                .map_err(|e| DomainError::General(e))?;

            if data.is_base64 {
                general_purpose::STANDARD_NO_PAD.encode(bytes)
            } else {
                match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(_) => "Use base64, decoded bytes are not valid UTF-8".into(),
                }
            }
        }
        CipherKindPayload::Vigener => todo!(),
    };

    Ok(Template::render(
        "decrypted",
        context! {
            text: res
        },
    ))
}

async fn get_cipher_id(
    db: &BrutusDb,
    user_id: i32,
    cipher_id: i32,
) -> Result<GetCeasarInternalsDto, DomainError> {
    let loc_user_id = user_id;
    let loc_cipher_id = cipher_id;

    db.run(move |conn| {
        use crate::schema::ceasar::dsl::*;
        use diesel::prelude::*;

        ceasar
            .filter(user_id.eq(loc_user_id))
            .filter(id.eq(loc_cipher_id))
            .select(GetCeasarInternalsDto::as_select())
            .first(conn)
    })
    .await
    .map_err(|_| DomainError::CipherNotFound)
}
