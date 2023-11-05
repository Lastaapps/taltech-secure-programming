use std::{
    env,
    time::{Duration, SystemTime},
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::domain::DomainError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: u64, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: u64, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

static ISSUER: &str = "Brutus";
static AUDIENCE_WEB: &str = "web";
static AUNDIENCES: [&str; 1] = [AUDIENCE_WEB];

lazy_static! {
    static ref HMAC_KEY: String = env::var("JWT_HMAC_KEY").unwrap();
    static ref HMAC_KEY_ENCODE: EncodingKey = EncodingKey::from_secret(HMAC_KEY.as_bytes());
    static ref HMAC_KEY_DECODE: DecodingKey = DecodingKey::from_secret(HMAC_KEY.as_bytes());
}

pub fn create_token(username: &str) -> Result<String, DomainError> {
    let claims = Claims::new(username);
    let token = encode(&Header::default(), &claims, &HMAC_KEY_ENCODE)?;
    println!("Created token:  {}", token);
    Ok(token)
}

pub fn verify_token(token: &str) -> Result<String, DomainError> {
    println!("Checking token: {}", token);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&AUNDIENCES);
    validation.set_issuer(&[ISSUER]);

    let token = decode::<Claims>(token, &HMAC_KEY_DECODE, &validation)?;
    let claims = token.claims;
    Ok(claims.sub)
}

impl Claims {
    fn new(username: &str) -> Self {
        // normalize the timestamps by stripping of microseconds
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let iat = now.as_secs();
        let exp = now
            .checked_add(Duration::new(60 * 10, 0))
            .unwrap()
            .as_secs();

        Self {
            aud: AUDIENCE_WEB.into(),
            exp,
            iat,
            iss: ISSUER.into(),
            nbf: iat,
            sub: username.to_owned(),
        }
    }
}
