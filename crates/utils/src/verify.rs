use crate::settings::structs::Settings;
use crate::token::*;
use actix_web::error as a_error;
use jsonwebtoken::{decode, errors::Error, errors::ErrorKind, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, Debug)]
#[serde(untagged)]
pub enum VerifyToken {
    VerifyState(VerifySS),
    VerifyAccess(VerifyAS),
    VerifyId(VerifyIS),
    VerifyRefresh(VerifyRS),
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct VerifySS {
    pub referrer_url: String,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct VerifyAS {
    pub client_id: String,
    pub client_secret: String,
    pub authorization_code: String,
    pub session: Option<String>,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct VerifyIS {
    pub client_id: String,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct VerifyRS {
    pub sub: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum TokenInfo {
    Active {
        active: bool,
        scope: bool,
        client_id: bool,
        sub: bool,
        exp: bool,
    },
    Inactive {
        active: bool,
    },
}

impl VerifyAS {
    pub fn verify(token: &str) -> Result<String, a_error::Error> {
        let config = Settings::get().config_at();

        let validation = Validation {
            leeway: config.validation.leeway,

            validate_exp: config.validation.validate_exp,
            validate_nbf: config.validation.validate_nbf,

            iss: config.validation.iss,
            sub: config.validation.sub,
            aud: None,

            algorithms: config
                .validation
                .algorithm
                .iter()
                .map(|x| algorithm(x.to_string()))
                .collect(),
        };

        // validation.set_audience(&config.validation.aud.unwrap());

        let struct_s = decode::<TokenAS>(
            token,
            &DecodingKey::from_secret(config.algorithm.private_key.as_ref()),
            &validation,
        );

        match &struct_s {
            Ok(_) => {
                let mut value = token.to_string();
                value.insert_str(0, "AT_");
                Ok(value)
            }

            Err(err) => custom_error(err.kind()),
        }
    }
}
impl VerifyIS {
    pub fn verify(token: &str) -> Result<String, a_error::Error> {
        let config = Settings::get().config_it();

        let validation = Validation {
            leeway: config.validation.leeway,

            validate_exp: config.validation.validate_exp,
            validate_nbf: config.validation.validate_nbf,

            iss: config.validation.iss,
            sub: config.validation.sub,
            aud: None,

            algorithms: config
                .validation
                .algorithm
                .iter()
                .map(|x| algorithm(x.to_string()))
                .collect(),
        };

        // validation.set_audience(&config.validation.aud.unwrap());

        let struct_s = decode::<TokenIS>(
            token,
            &DecodingKey::from_secret(config.algorithm.private_key.as_ref()),
            &validation,
        );

        match &struct_s {
            Ok(_) => {
                let mut value = token.to_string();
                value.insert_str(0, "IT_");
                Ok(value)
            }

            Err(err) => custom_error(err.kind()),
        }
    }
}
impl VerifyRS {
    pub fn verify(token: &str) -> Result<String, a_error::Error> {
        let config = Settings::get().config_rt();

        let validation = Validation {
            leeway: config.validation.leeway,

            validate_exp: config.validation.validate_exp,
            validate_nbf: config.validation.validate_nbf,

            iss: config.validation.iss,
            sub: config.validation.sub,
            aud: None,

            algorithms: config
                .validation
                .algorithm
                .iter()
                .map(|x| algorithm(x.to_string()))
                .collect(),
        };

        // validation.set_audience(&config.validation.aud.unwrap());

        let struct_s = decode::<TokenRS>(
            token,
            &DecodingKey::from_secret(config.algorithm.private_key.as_ref()),
            &validation,
        );

        match &struct_s {
            Ok(_) => {
                let mut value = token.to_string();
                value.insert_str(0, "RT_");
                Ok(value)
            }
            Err(err) => custom_error(err.kind()),
        }
    }
}

impl VerifySS {
    pub fn verify(token: &str) -> Result<String, a_error::Error> {
        // let mut result = well_formed(token);
        let config = Settings::get().config_st();

        let validation = Validation {
            leeway: config.validation.leeway,

            validate_exp: config.validation.validate_exp,
            validate_nbf: config.validation.validate_nbf,

            iss: config.validation.iss,
            sub: None,
            aud: None,

            algorithms: config
                .validation
                .algorithm
                .iter()
                .map(|x| algorithm(x.to_string()))
                .collect(),
        };

        // validation.set_audience(&config.validation.aud.unwrap());

        let struct_s: Result<jsonwebtoken::TokenData<TokenSS>, Error> = decode::<TokenSS>(
            token,
            &DecodingKey::from_secret(config.algorithm.private_key.as_ref()),
            &validation,
        );

        match &struct_s {
            Ok(_) => {
                let mut value = token.to_string();
                value.insert_str(0, "ST_");
                Ok(value)
            }
            Err(err) => custom_error(err.kind()),
        }
    }
}

pub fn algorithm(algorithm: String) -> Algorithm {
    match algorithm.as_str() {
        "HS256" => Algorithm::HS256,
        "HS512" => Algorithm::HS512,
        "RS256" => Algorithm::RS256,
        "RS512" => Algorithm::RS512,
        _ => panic!(),
    }
}

pub fn custom_error(error: &ErrorKind) -> Result<std::string::String, actix_web::Error> {
    match error {
        ErrorKind::ExpiredSignature => Err(a_error::ErrorRequestTimeout("Expired Signature")),
        ErrorKind::InvalidToken => Err(a_error::ErrorInternalServerError("Invalid Token")),
        ErrorKind::InvalidSignature => Err(a_error::ErrorInternalServerError("Invalid Signature")),
        ErrorKind::InvalidEcdsaKey => Err(a_error::ErrorInternalServerError("Invalid Ecdsa Key")),
        ErrorKind::InvalidRsaKey => Err(a_error::ErrorInternalServerError("Invalid Rsa Key")),
        ErrorKind::InvalidAlgorithmName => {
            Err(a_error::ErrorInternalServerError("Invalid Algorithm Name"))
        }
        ErrorKind::InvalidKeyFormat => Err(a_error::ErrorInternalServerError("Invalid Key Format")),
        ErrorKind::InvalidIssuer => Err(a_error::ErrorInternalServerError("Invalid Issuer")),
        ErrorKind::InvalidAudience => Err(a_error::ErrorInternalServerError("Invalid Audience")),
        ErrorKind::InvalidSubject => Err(a_error::ErrorInternalServerError("Invalid Subject")),
        ErrorKind::ImmatureSignature => {
            Err(a_error::ErrorInternalServerError("Immature Signature"))
        }
        ErrorKind::InvalidAlgorithm => Err(a_error::ErrorInternalServerError("Invalid Algorithm")),
        ErrorKind::Base64(_) => Err(a_error::ErrorInternalServerError("Base64")),
        ErrorKind::Json(_) => Err(a_error::ErrorInternalServerError("Json")),
        ErrorKind::Utf8(_) => Err(a_error::ErrorInternalServerError("Utf8")),
        ErrorKind::Crypto(_) => Err(a_error::ErrorInternalServerError("Crypto")),
        _ => panic!(),
    }
}

pub fn verify_token_info(token: &str) -> TokenInfo {
    let config = Settings::get().config_at();

    let validation = Validation {
        leeway: config.validation.leeway,

        validate_exp: config.validation.validate_exp,
        validate_nbf: config.validation.validate_nbf,

        iss: None,
        sub: None,
        aud: None,

        algorithms: config
            .validation
            .algorithm
            .iter()
            .map(|x| algorithm(x.to_string()))
            .collect(),
    };

    // validation.set_audience(&config.validation.aud.unwrap());

    let struct_s = decode::<TokenAS>(
        token,
        &DecodingKey::from_secret(config.algorithm.private_key.as_ref()),
        &validation,
    );
    let result: TokenInfo = match &struct_s {
        Ok(_) => TokenInfo::Active {
            active: true,
            scope: true,
            client_id: true,
            sub: true,
            exp: true,
        },

        Err(_err) => TokenInfo::Inactive { active: false },
    };
    result
}
