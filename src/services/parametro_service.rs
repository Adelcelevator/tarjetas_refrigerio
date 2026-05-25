use diesel_async::AsyncPgConnection;

use crate::{models::{data_model::postgres::parametro_det_model::{ParametroRes, ParametrosReq, ParametrosRes}, responses::response::ResponseData}, repository::postgres::parametro_repository::cargar_parametro, utils::enums::errors::service_error::ServiceError};

pub async fn cargar_parametros(con: &mut AsyncPgConnection,
                        parametros:ParametrosReq)-> Result<ResponseData<ParametrosRes>,ServiceError>{
    let mut respuesta: Vec<ParametroRes> = vec![];

    for parametro in parametros.busqueda_parametros{
        
        let params= cargar_parametro(con, parametro).await?;
        for para in params{
            let param_res = ParametroRes{
                estado: para.estado,
                id: para.id_parametro_cab,
                valor: para.parametro_valor,
                nombre: para.nombre
            };
            respuesta.push(param_res);
        }
    }
    let res = ParametrosRes{
        parametros:respuesta
    };
    Ok(ResponseData { codigo: Some(200), 
                   status: Some(String::from("OK")), 
                   mensaje: Some(String::from("Ok")), 
                   data: Some(res) 
                })
}
