use base64::{engine::general_purpose, Engine as _};
use std::ops::Rem;

use super::DomainError;

// -- Ceasar ------------------------------------------------------------------
fn ceasar(bytes: &mut [u8], shift: i64, encode: bool) {
    let shift = shift.rem(256) as u8;
    let shift = if encode {
        shift
    } else {
        0u8.wrapping_sub(shift)
    };

    for b in bytes.iter_mut() {
        *b = (*b).wrapping_add(shift);
    }
}

pub fn encode_ceasar(bytes: &mut [u8], shift: i64) -> Result<String, String> {
    ceasar(bytes, shift, true);
    Ok(general_purpose::STANDARD_NO_PAD.encode(bytes))
}

pub fn decode_ceasar(data: &str, shift: i64) -> Result<Vec<u8>, String> {
    let mut bytes = general_purpose::STANDARD_NO_PAD
        .decode(data)
        .map_err(|e| format!("Base decode failed: {}", e))?;

    ceasar(&mut bytes, shift, false);

    Ok(bytes)
}

// -- Vigener -----------------------------------------------------------------
fn validate_vigener_input(bytes: &[u8], key: &[u8]) -> Result<(), DomainError> {
    if bytes.len() != key.len() {
        Err(DomainError::VigenereKeyDifferentLength)?;
    };
    Ok(())
}

pub fn encode_vigener(bytes: &mut [u8], key: &[u8]) -> Result<(String, String), DomainError> {
    validate_vigener_input(bytes, key)?;

    bytes
        .iter_mut()
        .zip(key)
        .for_each(|(x, y)| *x = x.wrapping_add(*y));

    Ok((
        general_purpose::STANDARD_NO_PAD.encode(bytes),
        general_purpose::STANDARD_NO_PAD.encode(key),
    ))
}

pub fn decode_vigener(data: &str, key: &str) -> Result<Vec<u8>, DomainError> {
    let mut bytes = general_purpose::STANDARD_NO_PAD
        .decode(data)
        .map_err(|e| format!("Base decode failed: {}", e))?;
    let key = general_purpose::STANDARD_NO_PAD
        .decode(key)
        .map_err(|e| format!("Key base64 decode failed: {}", e))?;

    validate_vigener_input(&bytes, &key)?;

    bytes
        .iter_mut()
        .zip(key)
        .for_each(|(x, y)| *x = x.wrapping_sub(y));

    Ok(bytes)
}
