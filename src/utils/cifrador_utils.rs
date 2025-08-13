type Clave = aes_gcm::aead::generic_array::GenericArray<u8, sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UInt<sha3::digest::typenum::UTerm, aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B0>, aes_gcm::aead::consts::B0>, aes_gcm::aead::consts::B0>, aes_gcm::aead::consts::B0>, aes_gcm::aead::consts::B0>>;
use aes_gcm::{Aes256Gcm, Nonce, KeyInit, aead::Aead};
use base64::Engine;
use sha3::{Digest, Sha3_256};
use std::env;

pub fn cifrar(texto_cifrar: String) -> String {
    let cipher = Aes256Gcm::new_from_slice(_get_clave().as_ref()).expect("Error al generar el cifrador");
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(nonce, texto_cifrar.as_bytes()).expect("msg");
    base64::engine::general_purpose::STANDARD.encode(ciphertext)
}

pub fn descifrar(texto_descifrar: &String) -> String {
    let cipher = Aes256Gcm::new_from_slice(_get_clave().as_ref()).expect("Error al generar el cifrador");
    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
    let binding = base64::engine::general_purpose::STANDARD
        .decode(texto_descifrar.as_bytes())
        .expect("Error al formatear");
    let plaintext = cipher
        .decrypt(nonce, binding.as_slice())
        .expect("error al descifrar");
    std::string::String::from_utf8(plaintext).expect("Error al formatear la salida")
}

fn _get_clave() -> Clave {
    let clave_cifrado = env::var("CLAVECIFRADO").expect("No se encontro la variable CLAVECIFRADO");
    let mut hasher = Sha3_256::new();
    hasher.update(clave_cifrado.as_bytes());
    hasher.finalize()
}