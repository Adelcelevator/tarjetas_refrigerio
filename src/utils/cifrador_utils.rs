use aes_gcm::{Aes256Gcm, Nonce, KeyInit, aead::Aead};
use base64::Engine;
use rand::RngExt;
use sha3::{Digest, Sha3_256};
use log::error;
use crate::utils::{enums::errors::service_error::ServiceError, env_utils::get_variable};

pub fn cifrar(texto_cifrar: String) -> Result<String,ServiceError> {
    let cipher = match Aes256Gcm::new_from_slice(_get_clave()?.as_ref()){
        Ok(res)=>res,
        Err(error)=>{
            error!("Error al generar el cifrador: {}",error);
            return Err(ServiceError::InternalServerError);
        }
    };
    let mut proto_nonce = [0u8;12];
    rand::rng().fill(&mut proto_nonce);

    let nonce = Nonce::from(proto_nonce);
    let ciphertext = match cipher.encrypt(&nonce, texto_cifrar.as_bytes()){
        Ok(res)=>res,
        Err(error)=>{
            error!("Existio un error al cifrar el texto: {}",error);
            return Err(ServiceError::InternalServerError);
        }
    };

    let mut texto_final = Vec::with_capacity(proto_nonce.len()+ciphertext.len());

    texto_final.extend_from_slice(&proto_nonce);
    texto_final.extend_from_slice(&ciphertext);

    Ok(base64::engine::general_purpose::STANDARD.encode(texto_final))
}

pub fn descifrar(texto_descifrar: &String) -> Result<String,ServiceError> {
    let cipher = match Aes256Gcm::new_from_slice(_get_clave()?.as_ref()){
        Ok(res)=>res,
        Err(error)=>{
            error!("Existio  un error al generar el decifrador: {}",error);
            return Err(ServiceError::InternalServerError);
        }
    };
    let decodificado = match base64::engine::general_purpose::STANDARD
        .decode(texto_descifrar.as_bytes()){
            Ok(des)=>des,
            Err(error)=>{
                error!("Existio  un error al decodificar de base 64: {}",error);
                return Err(ServiceError::InternalServerError);
            }
        };
    
    if decodificado.len() < 12 {
        error!("El texto decodificado no es el texto esperado: {}",decodificado.len());
        return Err(ServiceError::InternalServerError);
    }
    let (nonce_bytes, ciphertext_bytes) = decodificado.split_at(12);
    let nonce = match nonce_bytes.try_into(){
        Ok(res)=>res,
        Err(error)=>{
            error!("No se pudo determinar el nonce: {}",error);
        return Err(ServiceError::InvalidToken("No se pudo descifrar el token".to_string()));
        }
    };
    let plaintext = match cipher
        .decrypt(nonce, ciphertext_bytes){
            Ok(descifrado)=>descifrado,
            Err(error)=>{
                error!("Existio  un error al descifrar: {}",error);
                return Err(ServiceError::InternalServerError);
            }
        };
    
    match std::string::String::from_utf8(plaintext){
        Ok(texto)=>Ok(texto),
        Err(error)=>{
            error!("Existio un error al formtear el texto: {}",error);
            Err(ServiceError::InternalServerError)
        }
    }
}

fn _get_clave()-> Result<sha3::digest::Output<Sha3_256>,ServiceError> {
    let Some(clave_cifrado) = get_variable::<String>("CLAVECIFRADO") else {
        error!("No se pudo detrminar la clave de cifrado.");
        return Err(ServiceError::InternalServerError);
    };
    let mut hasher = Sha3_256::new();
    hasher.update(clave_cifrado.as_bytes());
    Ok(hasher.finalize())
}