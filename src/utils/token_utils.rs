use jsonwebtoken::{
    decode, encode, errors::Result, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Serialize, de::DeserializeOwned};
use sha3::{Sha3_256, Digest};
use uuid::Uuid;
use log::error;

use crate::utils::env_utils::get_variable;

pub fn create_jwt<T:Serialize>(cont:T) -> Result<String> {
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some(Uuid::new_v4().to_string());
    let Some(clave) = clave_token() else{
        error!("No se pudo obtener la clave del token.");
        return Ok(String::new());
    };
    encode(
        &header,
        &cont,
        &EncodingKey::from_secret(&clave),
    )
}

pub fn decodificar_token<T:DeserializeOwned>(token: &str) -> std::result::Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error> {
    let Some(clave) = clave_token() else{
        error!("No se pudo obtener la clave del token.");
        return Err(jsonwebtoken::errors::new_error(jsonwebtoken::errors::ErrorKind::InvalidKeyFormat));
    };
    decode::<T>(
        token,
        &DecodingKey::from_secret(&clave),
        &Validation::new(Algorithm::HS512),
    )
}

fn clave_token() -> Option<sha3::digest::Output<Sha3_256>> {
    let Some(clave_token) = get_variable::<String>("CLAVETOKEN") else{
        error!("No se encontro la variable CLAVETOKEN");
        return None;
    };
    let mut hasher = Sha3_256::new();
    hasher.update(clave_token.as_bytes());
    Some(hasher.finalize())
}
