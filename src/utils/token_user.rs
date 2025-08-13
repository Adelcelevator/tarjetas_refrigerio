use chrono::Utc;
use jsonwebtoken::errors::Result;
use serde::{Deserialize, Serialize};
use std::env;
use log::info;

use super::{cifrador_utils, token_utils};

#[derive(Debug, Deserialize, Serialize)]
pub struct ClaimsUser {
    pub sub: String,
    pub exp: usize,
    pub roles: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TokenUser {
    pub contenido: Option<ClaimsUser>,
    pub token: Option<String>,
}

pub fn void_claims_user() -> ClaimsUser {
    ClaimsUser{
        sub:String::new(),
        exp:0,
        roles:vec![]
    }
}

pub fn create_jwt(usuario: &str,roles:&[Option<String>]) -> Result<String> {
    let expiration = _expiracion();
    let roles_vec:Vec<String> = roles.iter().filter_map(|rol| rol.clone()).collect();
    let cont = ClaimsUser {
        sub: String::from(usuario),
        exp: expiration as usize,
        roles: roles_vec
    };
    if cont.sub.is_empty() {
        return Ok(String::new());
    }
    token_utils::create_jwt::<ClaimsUser>(cont)
}

pub fn validar_token(token: &str) -> ClaimsUser {
    if !token.is_empty() {
        let decodificado = urlencoding::decode(token).expect("Error al decodificar de url").to_string();
        let token = cifrador_utils::descifrar(&decodificado);
        let res  = token_utils::decodificar_token::<ClaimsUser>(token.as_str());
        match res {
            Ok(claim)=>claim.claims,
            Err(er)=>{
                info!("Ocurrio un error al validar el token {}",er);
                void_claims_user()
            }
        }
    } else {
        ClaimsUser {
            exp: 0,
            sub: String::new(),
            roles: vec![]
        }
    }
}

fn _expiracion() -> i64 {
    let tiempo_expiracion = env::var("EXPIRACIONTOKENMINUTOS")
        .expect("No se encontro la variable EXPIRACIONTOKENMINUTOS");
    Utc::now()
        .checked_add_signed(chrono::TimeDelta::try_minutes(
            tiempo_expiracion
                .parse::<i64>()
                .expect("No se pudo transformar el tiempo de expiracion"),
        ).unwrap())
        .expect("valid timestamp")
        .timestamp()
}
