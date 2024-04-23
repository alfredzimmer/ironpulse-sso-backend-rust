use axum::extract::FromRef;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::APIError;

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalClaims {
    pub sub: String,
    pub exp: usize,
    pub uid: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthClaims {
    sub: String,
    exp: usize,
    client_id: String
}

#[derive(Clone)]
pub struct JwtHelper {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey
}

impl JwtHelper {
    pub fn new() -> JwtHelper {
        let doc = Ed25519KeyPair::generate_pkcs8(&SystemRandom::new()).unwrap();
        let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

        let pair = Ed25519KeyPair::from_pkcs8(doc.as_ref()).unwrap();
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        JwtHelper {
            encoding_key,
            decoding_key,
        }
    }

    pub fn encode(&self, claims: &impl Deserialize) -> String {
        jsonwebtoken::encode(&Header::new(Algorithm::EdDSA), claims, &self.encoding_key)
            .unwrap()
    }

    pub fn decode<T: Deserialize>(&self, token: &String) -> Result<T, APIError> {
        match jsonwebtoken::decode::<T>(token, &self.decoding_key, &Validation::new(Algorithm::EdDSA)) {
            Ok(data) => Ok(data.claims),
            Err(_) => Err(APIError::Unauthorized)
        }
    }
}

impl FromRef<AppState> for JwtHelper {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_helper.clone()
    }
}
