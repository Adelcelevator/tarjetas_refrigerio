type ClaveToken = aes_gcm::aead::generic_array::GenericArray<u8,sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UTerm,aes_gcm::aead::consts::B1,>,aes_gcm::aead::consts::B0,>,aes_gcm::aead::consts::B0,>,aes_gcm::aead::consts::B0,>,aes_gcm::aead::consts::B0>,aes_gcm::aead::consts::B0,>,>;
use jsonwebtoken::{
    decode, encode, errors::Result, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Serialize, de::DeserializeOwned};
use sha3::{Sha3_256, Digest};
use uuid::Uuid;
use std::env;

pub fn create_jwt<T:Serialize>(cont:T) -> Result<String> {
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some(Uuid::new_v4().to_string());
    encode(
        &header,
        &cont,
        &EncodingKey::from_secret(clave_token().as_ref()),
    )
}

pub fn decodificar_token<T:DeserializeOwned>(token: &str) -> std::result::Result<jsonwebtoken::TokenData<T>, jsonwebtoken::errors::Error> {
    decode::<T>(
        token,
        &DecodingKey::from_secret(clave_token().as_ref()),
        &Validation::new(Algorithm::HS512),
    )
}

fn clave_token() -> ClaveToken {
    let clave_token = env::var("CLAVETOKEN").expect("No se encontro la variable CLAVETOKEN");
    let mut hasher = Sha3_256::new();
    hasher.update(clave_token.as_bytes());
    hasher.finalize()
}
