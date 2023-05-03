use crate::settings::structs::{TokenAlgorithm};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

type Jwt = String;
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenSS {
    pub iss: String,
    pub exp: i64,
    pub referrer_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TokenAS {
    pub sub: String,
    pub client_id: String,
    pub scopes: Option<Vec<Scope>>,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenIS {
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenRS {
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TokenClaims {
    State(TokenSS),
    Access(TokenAS),
    Id(TokenIS),
    Refresh(TokenRS),
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Scope {
    Read,
    Write,
}
pub fn generate_token(
    claims: TokenClaims,
    algorithm: TokenAlgorithm,
) -> Result<Jwt, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(Algorithm::from_str(&algorithm.algorithm).unwrap()),
        &claims,
        &EncodingKey::from_secret(algorithm.private_key.as_ref()),
    )
}
