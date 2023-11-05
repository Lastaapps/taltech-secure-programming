use time::{
    format_description, macros::datetime, Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset,
};

use crate::domain::database::BrutusDb;
use crate::domain::DomainError;
use crate::models::UserIdDto;
use crate::schema;
use base64::{
    alphabet,
    engine::{general_purpose, DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
    Engine as _,
};

#[derive(FromFormField)]
pub enum CipherKindPayload {
    Ceasar,
    Vigener,
}

pub fn format_date_for_web(date: &OffsetDateTime) -> String {
    let format = format_description::well_known::Rfc3339;
    date.checked_to_offset(UtcOffset::UTC)
        .unwrap()
        .format(&format)
        .unwrap()
}

pub async fn get_user_id(db: &BrutusDb, username: &str) -> Result<i32, DomainError> {
    match db
        .run(|conn| {
            let loc_username = username.clone();
            use diesel::prelude::*;
            use schema::users::dsl::*;

            users
                .filter(username.eq(loc_username))
                .select(UserIdDto::as_select())
                .first(conn)
                .optional()
        })
        .await
        .unwrap()
        .map(|it| it.id)
    {
        Some(data) => Ok(data),
        None => Err(DomainError::General("User not found!".into())),
    }
}

lazy_static! {
    static ref BASE64_ENGINES: [GeneralPurpose; 2] = {
        let config: GeneralPurposeConfig =
            GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent);
        [
            GeneralPurpose::new(&alphabet::STANDARD, config.clone()),
            GeneralPurpose::new(&alphabet::URL_SAFE, config.clone()),
        ]
    };
}

pub fn decode_base64(data: &str) -> Result<Vec<u8>, DomainError> {
    for engine in BASE64_ENGINES.iter() {
        if let Ok(res) = engine.decode(data) {
            return Ok(res);
        }
    }

    Err(DomainError::NotBase64)
}

pub fn now_primitive() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(now.date(), now.time())
}
