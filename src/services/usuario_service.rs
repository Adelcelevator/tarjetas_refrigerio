use actix_web::web::Data;
use bcrypt::{hash, verify};
use chrono::Utc;
use diesel::{
    r2d2::{ConnectionManager, Pool,}, update, ExpressionMethods, PgConnection, RunQueryDsl
};
use log::error;

use crate::{models::{data_model::{token_model::Token, usuario_model::{ChangePassword, Login, UsuarioConsulta}}, responses::{login_response::{LoginResponse, RepresentadosResponse}, response::{Response, ResponseData}}}, repository::{persona_repository::{self, buscar_persona}, db_tarjetas_repository::db_tarjetas::tbl_usuario, representante_representado_repository, roles_repository, tarjeta_repository::cargar_tarjetas_usuario, token_repo::TokenRepo, usuario_repository}, utils::{bd_utils::get_conexion, cifrador_utils, enums::estados_enum::Estados, token_user::{self, validar_token}}};

pub async fn login_service(
    db_mongo: &Data<TokenRepo>,
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    login: Login,
) -> ResponseData<LoginResponse> {
    let conn = get_conexion(pool);
    if conn.is_none(){
        return ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None,
        };
    }
    let mut con = conn.unwrap();
    let usuario:UsuarioConsulta = usuario_repository::buscar_usuario(&mut con, login.usuario);

    if usuario.usu_id == None {
        return ResponseData {
            codigo: Some(404),
            status: Some(String::from("Usuario no encontrado")),
            mensaje: Some(String::from("Usuario no encontrado")),
            data: None,
        };
    }
    if usuario.estado.is_some() &&
       usuario.estado.unwrap() != Estados::Activo.to_string() {
        return ResponseData {
            codigo: Some(403),
            status: Some(String::from("Usuario no activo")),
            mensaje: Some(String::from("Su usuario no se encuentra activo")),
            data: None,
        };
       }

    if usuario.usu_contra.is_some() {
        let verificacion = match verify(login.clave, &usuario.usu_contra.unwrap()){
            Ok(ver)=>ver,
            Err(error)=>{
                error!("Existio un error al validar la clave: {}",error);
                false
            }
        };

        if !verificacion {
            return ResponseData {
                codigo: Some(403),
                status: Some(String::from("Clave incorrecta")),
                mensaje: Some(String::from("Su clave es incorrecta")),
                data: None,
            };
        }
        
        let persona = persona_repository::buscar_persona(&mut con, &usuario.per_id.unwrap());
        let roles: Vec<Option<String>> = roles_repository::roles_usuario(&mut con, &usuario.usu_id.unwrap());
        if  roles.is_empty() || 
            roles[0].is_none() {
            return ResponseData {
                codigo: Some(403),
                status: None,
                mensaje: Some(String::from("El usuario no tiene roles asignados")),
                data: None,
            };
        }
        let usu_cop = usuario.usu_usuario.unwrap();
        let repre = representante_representado_repository::cargar_representado_por_usuario(&mut con, &usu_cop);
        let tk = match token_user::create_jwt(&usu_cop,&roles) {
            Ok(res) => res,
            Err(error) => {
                error!("Existio un error al generar el token: {}",error);
                String::new()
            },
        };
        if tk == *"" {
            return ResponseData {
                codigo: Some(500),
                status: Some(String::from("Error interno del servidor")),
                mensaje: Some(String::from("No se pudo generar el token")),
                data: None,
            };
        }
        let tk = urlencoding::encode(cifrador_utils::cifrar(tk).as_str()).to_string();
        let tk_clone = tk.clone();
        let mongo_token = Token {
            id: None,
            token: tk,
            usuario: usu_cop,
            estado: Estados::Activo,
            fecha_registro: Utc::now().timestamp(),
        };
        db_mongo
            .guardar_token(mongo_token)
            .await
            .expect("Error al guardar en la mongo");
        let mut representantes = vec![];
        for repres in repre{
            let persona =buscar_persona(&mut con,&repres.repsentado_id.unwrap()); 
            representantes.push( RepresentadosResponse{
                id: repres.repre_id.unwrap(),
                nombre: persona.per_nombre,
            });
        }

        let tarjetas = cargar_tarjetas_usuario(&mut con, &persona.per_id.unwrap());
        let respuesta:LoginResponse = LoginResponse{
            token: tk_clone,
            nombre: persona.per_nombre,
            roles,
            representados: Some( representantes ),
            tarjetas: Some(tarjetas)
        };
        ResponseData {
            codigo: Some(200),
            status: Some(String::from("OK")),
            mensaje: Some(String::from("Exitoso")),
            data: Some(respuesta),
        }
    } else {
        ResponseData {
            codigo: Some(401),
            status: Some(String::from("Clave incorrecta")),
            mensaje: Some(String::from("Su clave es incorrecta")),
            data: None,
        }
    }
}

pub async fn logout_service(db_mongo: &Data<TokenRepo>, token: &str) -> Response {
    let token = urlencoding::encode(token).to_string();
    let bor = db_mongo.borrar_token(token.as_str()).await;
    match bor {
        Ok(_) => Response {
            codigo: Some(200),
            status: Some(String::from("OK")),
            mensaje: None,
        },
        Err(err) => {
            error!("Existio un error al borrar el token: {}", err);
            Response {
                codigo: Some(500),
                status: Some(String::from("Error interno del servidor")),
                mensaje: Some(String::from("Exisitio un problema al borrar el token")),
            }
        }
    }
}

pub async fn change_password_service(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    data_change: ChangePassword,
) -> Response {    
    let conn = get_conexion(pool);
    if conn.is_none(){
        return Response {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
        };
    }
    let mut con = conn.unwrap();

    let contenido = validar_token(&data_change.token);
    let hashed_password = match hash(data_change.clave, 10){
        Ok(hashed)=>hashed,
        Err(error)=>{
            error!("Existio un error al hashear la clave: {}",error);
            String::new()
        },
    };
    if hashed_password.is_empty() {
        return Response{
            codigo: Some(403),
            status: None,
            mensaje: Some(String::from("La clave no se ha podido cifrar")),
        };
    }
    let resultado = update(tbl_usuario::dsl::tbl_usuario)
          .filter(tbl_usuario::usu_usuario.eq(contenido.sub))
          .set(tbl_usuario::usu_contra.eq(hashed_password))
          .execute(&mut con);
    
    let actualizacion = match resultado {
        Ok(res)=> res,
        Err(error) =>{
            error!("Error al actualizar la clave: {}",error);
            0
        },
    };

    if actualizacion == 0{
        return Response{
            codigo: Some(500),
            status: None,
            mensaje: Some(String::from("Existio un error al calmbiar la clave")),
        };
    }
    Response{
        codigo: Some(200),
        status: None,
        mensaje: Some(String::from("Exito al cambiar la clave")),
    }
}
