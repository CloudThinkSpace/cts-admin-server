use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

const SECRET: &str = "CloudThinkSpace";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims<T>
{
    pub user: T,
    pub exp: u64,
}

impl<T> Claims<T>
    where T: DeserializeOwned + Serialize
{
    pub fn new(user: T, exp: u64) -> Self {
        Self {
            user,
            exp,
        }
    }
}

pub fn encode_token<T>(user: T, exp: u64) -> Result<String, String>
    where T: Serialize
{
    let token = encode(
        &Header::default(),
        &Claims {
            user,
            exp
        },
        &EncodingKey::from_secret(SECRET.as_bytes()),
    ).map_err(|err| {
        err.to_string()
    })?;
    Ok(token)
}

pub fn decode_token<T>(token: &str) -> Result<Claims<T>, String>
    where T: DeserializeOwned
{
    let mut  validation = Validation::default();
    validation.validate_exp = true;
    validation.leeway = 0;
    let token_data = decode::<Claims<T>>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &validation,
    ).map_err(|err| err.to_string())?;

    Ok(token_data.claims)
}
