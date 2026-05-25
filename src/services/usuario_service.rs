use bcrypt::{hash, verify};
use chrono::Utc;
use diesel::{ExpressionMethods, update};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;
use mongodb::Client;

use crate::{models::{data_model::{mongo::token_model::Token, postgres::usuario_model::{ChangePassword, Login}}, responses::{login_response::{LoginResponse, RepresentadosResponse}, response::{Response, ResponseData}}}, repository::{mongo::token_repo::{borrar_token, guardar_token}, postgres::{db_tarjetas_repository::db_tarjetas::tbl_usuario, persona_repository::buscar_persona, representante_representado_repository::cargar_representado_por_usuario, roles_repository::roles_usuario, tarjeta_repository::cargar_tarjetas_usuario, usuario_repository::buscar_usuario}}, utils::{cifrador_utils::cifrar, enums::{errors::service_error::ServiceError, estados_enum::Estados}, token_user::{create_jwt, validar_token}}};

pub async fn login_service(
    cliente: Client,
    con: &mut AsyncPgConnection,
    login: Login,
) -> Result<ResponseData<LoginResponse>,ServiceError> {

    let usuario = buscar_usuario(con, login.usuario).await?;

    if usuario.usu_id.is_none() {
        return Err(ServiceError::NotFound("Usuario no encontrado.".to_string()));
    }

    let Some(estado) = usuario.estado else{
        return Err(ServiceError::ValidationError("Su usuario no tiene un estado procesable.".to_string()));
    };

    let Some(clave) = usuario.usu_contra else{
        return Err(ServiceError::ValidationError("Su usuario no tiene permisos para acceder".to_string()));
    };
    if estado != Estados::Activo.to_string() {
        return Err(ServiceError::ValidationError("Su usuario no se encuentra activo.".to_string()));
       }

    let verificacion = match verify(login.clave, &clave){
        Ok(ver)=>ver,
        Err(error)=>{
            error!("Existio un error al validar la clave: {}",error);
            false
        }
    };

    if !verificacion {
        return Err(ServiceError::ValidationError("Su clave es incorrecta.".to_string()));
    }
    let Some(per_id) = usuario.per_id else{
        return Err(ServiceError::PersonaNoEncontrada("No se pudo determinar a la persona.".to_string()));
    };
    
    let per = buscar_persona(con, &per_id).await?;
    let Some(usu_id) = usuario.usu_id else{
        return Err(ServiceError::PersonaNoEncontrada("No se pudo determinar un usuario.".to_string()));
    };
    let roles = roles_usuario(con, &usu_id).await?;
    if  roles.is_empty() || 
        roles[0].is_none() {
        return Err(ServiceError::NotFound("El usuario no tiene roles asignados.".to_string()));
    }
    let Some(usu_cop) = usuario.usu_usuario else{
        return Err(ServiceError::NotFound("No se pudo determinar el usuario.".to_string()));
    };
    let repre = cargar_representado_por_usuario(con, &usu_cop).await?;
    let tk = match create_jwt(&usu_cop,&roles) {
        Ok(res) => res,
        Err(error) => {
            error!("Existio un error al generar el token: {}",error);
            String::new()
        },
    };
    if tk.is_empty() {
        return Err(ServiceError::InvalidToken("No se pudo generar el token.".to_string()));
    }
    let tk = urlencoding::encode(cifrar(tk)?.as_str()).to_string();

    let mongo_token = Token {
        id: None,
        token: tk.clone(),
        usuario: usu_cop,
        estado: Estados::Activo,
        fecha_registro: Utc::now().timestamp(),
    };

    guardar_token(cliente,mongo_token)
    .await?
    ;

    let mut representantes = vec![];
    for repres in repre{
        let Some(repsentado_id) = repres.repsentado_id else {
            return Err(ServiceError::NotFound("No se pudo definir bien a los representados.".to_string()));
        };
        let persona =buscar_persona(con,&repsentado_id).await?;
        
        let Some(repre_id)=repres.repre_id else {
            return Err(ServiceError::NotFound("No se pudo definir bien a los representados.".to_string()));
        };

        representantes.push( RepresentadosResponse{
            id:repre_id,
            nombre: persona.per_nombre,
        });
    }

    let tarjetas = cargar_tarjetas_usuario(con, &per_id).await?;
    let respuesta:LoginResponse = LoginResponse{
        token: tk,
        nombre: per.per_nombre,
        roles,
        representados: representantes,
        tarjetas: tarjetas,
    };
    Ok(ResponseData {
        codigo: Some(200),
        status: Some(String::from("OK")),
        mensaje: Some(String::from("Exitoso")),
        data: Some(respuesta),
    })
}

pub async fn logout_service(cliente: Client, token: &str) -> Result<Response,ServiceError> {
    let token = urlencoding::encode(token).to_string();
    let bor = borrar_token(cliente,token.as_str()).await?;
    
    if bor.modified_count > 0 {
        return Ok(Response {
            codigo: Some(200),
            status: Some(String::from("OK")),
            mensaje: Some(String::new()),
        });
    }
    error!("Existio un error al borrar el token.");
    Err(ServiceError::InvalidToken("Existio un error al administrar el token.".to_string()))
}

pub async fn change_password_service(
    con: &mut AsyncPgConnection,
    data_change: ChangePassword,
) -> Result<Response, ServiceError> {
    let contenido = validar_token(&data_change.token)?;
    let hashed_password = match hash(data_change.clave, 10){
        Ok(hashed)=>hashed,
        Err(error)=>{
            error!("Existio un error al hashear la clave: {}",error);
            return Err(ServiceError::InternalServerError);
        },
    };

    //TODO PASAR A LA CAPA REPOSITORY
    let resultado = update(tbl_usuario::dsl::tbl_usuario)
          .filter(tbl_usuario::usu_usuario.eq(contenido.sub))
          .set(tbl_usuario::usu_contra.eq(hashed_password))
          .execute(con).await;
    
    let actualizacion = match resultado {
        Ok(res)=> res,
        Err(error) =>{
            error!("Error al actualizar la clave: {}",error);
            0
        },
    };

    if actualizacion == 0 {
        return Err(ServiceError::InternalServerError);
    }
    Ok(Response{
        codigo: Some(200),
        status: None,
        mensaje: Some(String::from("Exito al cambiar la clave")),
    })
}
