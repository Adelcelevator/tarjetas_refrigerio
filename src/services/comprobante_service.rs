use std::path::Path;

use base64::{prelude::BASE64_STANDARD, Engine};

use chrono::Local;
use diesel_async::AsyncPgConnection;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use sha3::{Digest, Sha3_512};
use tokio::fs;

use log::error;

use crate::{models::{data_model::postgres::{comprobante_model::{AutorizarComprobante, ComprobanteConsulta, ComprobanteNuevoReq, ComprobanteReq, InsertarComprobante}, comprobante_representado_model::ComprobanteRepresentado}, responses::response::{Response, ResponseData}}, repository::postgres::{comprobante_repository::{autorizar_comprobantes, cargar_comprobante_por_estado, cargar_comprobante_por_numero_persona, cargar_comprobante_por_usuario, nuevo_comprobante}, comprobante_representado_repository::guardar_comprobante_representado, persona_repository::buscar_persona_por_usuario}, utils::{enums::{errors::service_error::ServiceError, estados_enum::Estados}, env_utils::get_variable}};

pub async fn guardar_service(
    con: &mut AsyncPgConnection,
    comprobante: ComprobanteNuevoReq,
) -> Result<Response,ServiceError> {
    let persona_buscada = buscar_persona_por_usuario(con, &comprobante.usuario).await?;

    let Some(path_comprobante) = get_variable::<String>("PATH_GUARDADO_COMPROBANTES") else {
        error!("Existio un error al leer la variable PATH_GUARDADO_COMPROBANTES");
        return Err(ServiceError::ConfiguracionError("No se pudo determinar la variable de guardado.".to_string()));
    };
    if persona_buscada.per_id == None {
        return Err(ServiceError::PersonaNoEncontrada("Usuario no encontrado".to_string()));
    }
    let contenido = comprobante.file.contenido;
    if contenido.is_empty() {
        return Err(ServiceError::ComprobanteInvalido("No existe un comprobante valido".to_string()));
    }
    
    let Some(id_per_buscada) = persona_buscada.per_id.clone()else{
        return Err(ServiceError::ComprobanteInvalido("No se puede asignar el comprobante.".to_string()));
    };

    let pre_busqueda = cargar_comprobante_por_numero_persona(con, (&comprobante.numero,&id_per_buscada)).await?;

    if !pre_busqueda.is_empty(){
        return Err(ServiceError::ComprobanteExiste);
    }

    let cont = match BASE64_STANDARD.decode(contenido){
        Ok(res)=>res,
        Err(error)=>{
            error!("Existio un error al decodificar el comprobante: {}",error);
            return Err(ServiceError::Base64DecodeError);
        }
    };

    let mut path = format!("{}{}",path_comprobante,&persona_buscada.per_identificacion);

    if !Path::new(&path).exists() {
        match fs::create_dir_all(&path).await{
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

    let guardado = match fs::write(&path, cont).await{
        Ok(_)=>true,
        Err(error)=>{
            error!("Existio un error al guardar el archivo: {}",error);
            false
        }
    };

    if !guardado{
        return Err(ServiceError::ComprobanteInvalido("Existio un problema al escribir el comprobante.".to_string()));
    }
    let Some(per_id) = persona_buscada.per_id else {
        return Err(ServiceError::ComprobanteInvalido("Existio un problema al guardar el comprobante.".to_string()));
    };

    let Some(valor_comprobante) = Decimal::from_f32(comprobante.valor) else {
        return Err(ServiceError::ComprobanteInvalido("Existio un error al transformar el valor a decimal.".to_string()));
    };

    let comp_guardar = InsertarComprobante{
                        comp_id: None,
                        comp_numero: comprobante.numero,
                        comp_path_fisico:path,
                        per_id: Some(per_id),
                        comp_valor: valor_comprobante,
                        estado: Estados::PorAutorizar.to_string(),
                        fe_creacion: Local::now().naive_local(),
                        usr_creacion: comprobante.usuario.clone(),
                        fe_modificacion:None,
                        usr_modificacion:None,
                    };

    let Some(guardar) = nuevo_comprobante(con, comp_guardar).await? else{
        return Err(ServiceError::ComprobanteInvalido("Existio un problema al guardar el comprobante.".to_string()));
    };

    if guardar == 0 {
        return Err(ServiceError::ComprobanteInvalido("Existio un problema al guardar el comprobante.".to_string()));
    }

    for repre in comprobante.representados {
        let comprobante_repre_guardar = ComprobanteRepresentado{
            comprobante_representado_id: None,
            repre_id: repre,
            comp_id: guardar.clone(),
            estado: Estados::PorAutorizar.to_string(),
            usr_creacion: comprobante.usuario.clone(),
            fe_creacion: Local::now().naive_local(),
            usr_modificacion: None,
            fe_modificacion: None,
        };
        let guardo = guardar_comprobante_representado(con, comprobante_repre_guardar).await?;

        if guardo == 0 {
            error!("Existio un error al guardar el comprobante {}, con el representado {}",repre,*&guardar);
        }
    }

    Ok(Response{
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("El comprobante se guardo con exito")),
    })
}

pub async fn cargar_comp_usuario_service(con:&mut AsyncPgConnection,
                                        peticion:ComprobanteReq
) -> Result<ResponseData<Vec<ComprobanteConsulta>>,ServiceError> {

    let persona = buscar_persona_por_usuario(con, &peticion.usuario).await?;

    let Some(per_id) = persona.per_id else{
        return Err(ServiceError::NotFound("No se encontro a la persona.".to_string()));
    };

    let encontrado = cargar_comprobante_por_usuario(con, &per_id).await?;
    let respuesta = ResponseData{
        codigo:Some(200),
        status:(Some(String::from("OK"))),
        mensaje:Some(String::from("Carga correcta")),
        data:Some(encontrado)
    };
    Ok(respuesta)
}

pub async fn cargar_comp_autorizar_service(con:&mut AsyncPgConnection
) -> Result<ResponseData<Vec<ComprobanteConsulta>>,ServiceError> {

    let encontrado = cargar_comprobante_por_estado(con, Estados::PorAutorizar).await?;
    Ok(ResponseData{
        codigo:Some(200),
        status:(Some(String::from("OK"))),
        mensaje:Some(String::from("Carga correcta")),
        data:Some(encontrado)
    })
}

pub async fn autorizar_service(
    con: &mut AsyncPgConnection,
    peticion: AutorizarComprobante,
) -> Result<Response, ServiceError> {  

    let aprobar = autorizar_comprobantes(con, peticion).await;

    Ok(Response{
        codigo:Some(200),
        status:Some(String::from("OK")),
        mensaje:Some(format!("Se aprobaron: {} comprobantes, no se pudieron aprobar {} comprobantes.",aprobar.0,aprobar.1)),
    })
}