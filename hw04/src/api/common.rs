use time::{format_description, macros::datetime, Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

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
