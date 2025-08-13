use std::{env, fs, path::Path};
use actix_web::web::Data;
use base64::{prelude::BASE64_STANDARD, Engine};

use chrono::Local;
use diesel::{
    r2d2::{ConnectionManager, Pool,}, PgConnection,
};
use dotenvy::dotenv;
use sha3::{Digest, Sha3_512};
use crate::{models::{data_model::{comprobante_model::{AutorizarComprobante, ComprobanteConsulta, ComprobanteNuevo, ComprobanteNuevoReq, ComprobanteReq}, comprobante_representado_model::ComprobanteRepresentado}, responses::response::{Response, ResponseData}}, repository::{self, comprobante_repository::{self, cargar_comprobante_por_numero_persona}, comprobante_representado_repository::guardar_comprobante_representado, persona_repository::buscar_persona_por_usuario}, utils::{bd_utils::get_conexion, enums::estados_enum::Estados}};

use log::error;

pub async fn guardar_service(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    comprobante: ComprobanteNuevoReq,
) -> Response {
    dotenv().ok();
    let conn = get_conexion(pool);
    if conn.is_none(){
        return Response {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
        };
    }
    let mut con = conn.unwrap();
    let persona_buscada = repository::persona_repository::buscar_persona_por_usuario(&mut con, &comprobante.usuario);
    let path_comprobante = match env::var("PATH_GUARDADO_COMPROBANTES") {
        Ok(v) => v.to_string(),
        Err(err) =>{
            error!("Existio un error al leer la variable PATH_GUARDADO_COMPROBANTES {}",err);
            "Error cargando las variables de sesion".to_string()
        }
    };
    if persona_buscada.per_id == None {
        return Response{
            codigo: Some(500),
            status: Some(String::from("No se pudo encontrar a la persona")),
            mensaje: Some(String::from("No se pudo encontrar a la persona a la que se le va a asignar el comprobante"))
        };
    }
    let contenido = comprobante.file.contenido;
    if contenido.is_empty() {
        return Response{
            codigo: Some(400),
            status: Some(String::from("Comprobante Vacio")),
            mensaje: Some(String::from("No se ha adjuntado un comprobante"))
        };
    }

    let pre_busqueda = cargar_comprobante_por_numero_persona(&mut con, (comprobante.numero.clone(),persona_buscada.per_id.unwrap().clone()));

    if !pre_busqueda.comp_numero.is_empty() {
        return Response{
            codigo: Some(400),
            status: Some(String::from("Comprobante Vacio")),
            mensaje: Some(String::from("El comprobante ya existe."))
        };
    }

    let cont = BASE64_STANDARD.decode(contenido).expect("No se pudo procesar el comprobante");
    let mut path = format!("{}{}",path_comprobante,&persona_buscada.per_identificacion);

    if !Path::new(&path).exists() {
        match fs::create_dir_all(&path){
            Ok(res)=>res,
            Err(error)=>{
                error!("Existio un error al guardar los comprobantes: {}",error);
            }
        };
    }
    let mut hasher =Sha3_512::new();
    hasher.update(Local::now().to_string().as_bytes());
    let resultado = hasher.finalize();
    let mut nombre = BASE64_STANDARD.encode(resultado);
    nombre = nombre.replace("/", "").replace("+", "").replace(".", "").replace("=", "");
    path = format!("{}/{}.{}",path,nombre,comprobante.file.extension);

    let guardado = match fs::write(&path, cont){
        Ok(_)=>true,
        Err(error)=>{
            error!("Existio un error al guardar el archivo: {}",error);
            false
        }
    };

    if !guardado{
        return Response{
            codigo: Some(500),
            status: Some(String::from("Error al guardar el comprobante")),
            mensaje: Some(String::from("Existio un problema al escribir el comprobante")),
        }
    }
    let comp_guardar: ComprobanteNuevo = ComprobanteNuevo{
        usuario: comprobante.usuario.clone(),
        numero: comprobante.numero,
        valor: comprobante.valor,
        file: path,
        persona_id: persona_buscada.per_id.unwrap(),
    };

    let guardar = comprobante_repository::nuevo_comprobante(&mut con, comp_guardar);

    if guardar == 0 {
        return Response{
            codigo: Some(500),
            status: Some(String::from("Error al guardar el comprobante")),
            mensaje: Some(String::from("Existio un problema al guardar el comprobante")),
        }
    }

    for repre in comprobante.representados {
        let comprobante_repre_guardar = ComprobanteRepresentado{
            comprobante_representado_id: None,
            repre_id: repre,
            comp_id: *&guardar,
            estado: Estados::PorAutorizar.to_string(),
            usr_creacion: comprobante.usuario.clone(),
            fe_creacion: Local::now().naive_local(),
            usr_modificacion: None,
            fe_modificacion: None,
        };
        let guardo = guardar_comprobante_representado(&mut con, comprobante_repre_guardar);

        if guardo == 0 {
            error!("Existio un error al guardar el comprobante {}, con el representado {}",repre,*&guardar);
        }
    }

    Response{
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("El comprobante se guardo con exito")),
    }
}

pub async fn cargar_comp_usuario_service(pool:&Data<Pool<ConnectionManager<PgConnection>>>,
                                        peticion:ComprobanteReq
) -> Option<ResponseData<Vec<ComprobanteConsulta>>> {

    let conn = get_conexion(pool);
    if conn.is_none(){
        return Some(ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None,
        });
    }
    let mut con = conn.unwrap();
    let persona = buscar_persona_por_usuario(&mut con, &peticion.usuario);
    let encontrado = comprobante_repository::cargar_comprobante_por_usuario(&mut con, &persona.per_id.unwrap());
    let respuesta = ResponseData{
        codigo:Some(200),
        status:(Some(String::from("OK"))),
        mensaje:Some(String::from("Carga correcta")),
        data:Some(encontrado)
    };
    Some(respuesta)
}

pub async fn cargar_comp_autorizar_service(pool:&Data<Pool<ConnectionManager<PgConnection>>>
) -> Option<ResponseData<Vec<ComprobanteConsulta>>> {

    let conn = get_conexion(pool);
    if conn.is_none(){
        return Some(ResponseData {
            codigo: Some(500),
            status: Some(String::from("Existio un error interno")),
            mensaje: Some(String::from("Existio un error interno")),
            data: None,
        });
    }
    let mut con = conn.unwrap();
    let encontrado = comprobante_repository::cargar_comprobante_por_estado(&mut con, Estados::PorAutorizar);
    let respuesta = ResponseData{
        codigo:Some(200),
        status:(Some(String::from("OK"))),
        mensaje:Some(String::from("Carga correcta")),
        data:Some(encontrado)
    };
    Some(respuesta)
}

pub async fn autorizar_service(
    pool: &Data<Pool<ConnectionManager<PgConnection>>>,
    peticion: AutorizarComprobante,
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
    let mut numeros_comprobantes:Vec<(String,String)> = Vec::new();

    for c in peticion.numeros {
        numeros_comprobantes.push((c,peticion.usuario.clone()));
    }

    let aprobar = comprobante_repository::autorizar_comprobantes(&mut con, numeros_comprobantes);

    Response{
        codigo:Some(200),
        status:Some(String::from("OK")),
        mensaje:Some(format!("Se aprobaron: {} comprobantes, no se pudieron aprobar {} comprobantes.",aprobar.0,aprobar.1)),
    }
}