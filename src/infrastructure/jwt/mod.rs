use std::env;

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::Serialize;

use crate::infrastructure::jwt::user_claims::UserClaims;

pub mod user_claims;

pub struct JwtService {}

impl JwtService {
    pub fn generate<T: Serialize>(claims: &T) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::new(Algorithm::HS256),
            claims,
            &EncodingKey::from_secret(
                env::var("JWT_SECRET")
                    .expect("JWT_SECRET is not defined")
                    .as_bytes(),
            ),
        )
    }

    pub fn verify(token: &str) -> Result<TokenData<UserClaims>, jsonwebtoken::errors::Error> {
        decode::<UserClaims>(
            token,
            &DecodingKey::from_secret(
                env::var("JWT_SECRET")
                    .expect("JWT_SECRET is not defined")
                    .as_bytes(),
            ),
            &Validation::new(Algorithm::HS256),
        )
    }
}
