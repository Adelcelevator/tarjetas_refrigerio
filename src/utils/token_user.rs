use chrono::{Duration, Utc};
use jsonwebtoken::errors::Result;
use serde::{Deserialize, Serialize};
use log::{error, info};

use crate::utils::{enums::errors::service_error::ServiceError, env_utils::get_variable};

use super::{cifrador_utils, token_utils};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimsUser {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<String>,
}

pub fn create_jwt(usuario: &str,roles:&[Option<String>]) -> Result<String> {
    let roles_vec:Vec<String> = roles.iter().filter_map(|rol| rol.clone()).collect();
    let cont = ClaimsUser {
        sub: String::from(usuario),
        exp: expiracion_token() as usize,
        roles: roles_vec
    };
    if cont.sub.is_empty() {
        return Ok(String::new());
    }
    token_utils::create_jwt::<ClaimsUser>(cont)
}

pub fn validar_token(token: &str) -> std::result::Result<ClaimsUser,ServiceError> {
    if token.is_empty() {
       return Err(ServiceError::InternalServerError);
    }
    let decodificado = match urlencoding::decode(token){
        Ok(res)=>res.to_string(),
        Err(error)=>{
            error!("Existio un error al decodificar de url: {}",error);
            return Err(ServiceError::InternalServerError);
        }
    };
        let token = cifrador_utils::descifrar(&decodificado)?;
        let res  = token_utils::decodificar_token::<ClaimsUser>(token.as_str());
        match res {
            Ok(claim)=>Ok(claim.claims),
            Err(er)=>{
                info!("Ocurrio un error al validar el token {}",er);
                return Err(ServiceError::InternalServerError);
            }
        }
}

pub fn expiracion_token()->i64{
    let Some(tiempo) = get_variable::<i64>("EXPIRACIONTOKENMINUTOS") else{
        error!("No se pudo recuperar la variable de tiempo para el token.");
        return 0;
    };

    let expira  = Utc::now() + Duration::minutes(tiempo);
    expira.timestamp()
}
