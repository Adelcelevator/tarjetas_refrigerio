use chrono::Local;
use diesel_async::AsyncPgConnection;
use rust_decimal::{Decimal, prelude::FromPrimitive,dec};
use log::error;

use crate::{models::{data_model::postgres::{detallle_pago_model::DetallePago, pago_model::{Pago, PagoReq}, persona_model::{BuscarPersonaCobro, PersonaCobroConsulta}}, responses::{response::{Response, ResponseData}, tipos_pagos_unidades_response::TiposPagosUnidadesResponse}}, repository::postgres::{detalle_pago_repository::guardar_detalle, pago_repository::guardar_pago_repo, persona_repository::buscar_persona_para_cobro, tarjeta_repository::cargar_saldo_tarjeta, tipo_pago_repository::get_tipos_pagos, unidades_repository::get_unidades}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

pub async fn get_tipos_pago_unidades(con:&mut AsyncPgConnection)->Result<ResponseData<TiposPagosUnidadesResponse>,ServiceError>{
    let tipos_pagos = get_tipos_pagos(con).await?;
    let unidades = get_unidades(con).await?;
    if tipos_pagos.is_empty() ||
       unidades.is_empty() {
        return Err(ServiceError::NotFound("No se encontro los tipos de pagos o las unidades".to_string()));
    }
    let res = TiposPagosUnidadesResponse{
        tipos_pagos,
        unidades
    };
    Ok(ResponseData {
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("Exito")),
        data: Some(res),
    })
}

pub async fn buscar_persona_cobro(con:&mut AsyncPgConnection,
                            buscar:BuscarPersonaCobro )->Result<ResponseData<Vec<PersonaCobroConsulta>>,ServiceError>{
    
    let encontrado = buscar_persona_para_cobro(con, buscar).await?;

    Ok(ResponseData {
        codigo: Some(200),
        status: Some(String::from("Exito")),
        mensaje: Some(String::from("Exito")),
        data: Some(encontrado),
    })
}

pub async fn guardar_pago(con:&mut AsyncPgConnection,
                          guardar:PagoReq )->Result<Response,ServiceError>{

    con.build_transaction().read_write().run::<Response,ServiceError,_>(async |tx|{
        let mut detalle_guardar:Vec<DetallePago> =vec![]; 
    
        let mut valor  = dec!(0);

        for dato in guardar.detalle_pagos.clone() {
            let Some(val) = Decimal::from_f32(dato.detalle_pago_valor) else{
                return Err(ServiceError::ValidationError("No se puedo transformar los valores del detalle.".to_string()));
            };
            valor = valor + val;
        }

        let Some(cobrar) = Decimal::from_f32(guardar.cobrar) else {
            return Err(ServiceError::ValidationError("No se puedo transformar el valor total a cobrar.".to_string()));
        };

        if valor != cobrar {
            return Err(ServiceError::ValidationError("El valor total del detalle no cuadra con el valor a guardar.".to_string()));
        }

        let pago = Pago{
            pago_id:None,
            per_id: Some(guardar.per_id.clone()),
            pago_valor_total: valor,
            pago_observacion: guardar.observacion,
            estado: Estados::Activo.to_string(),
            usr_creacion:guardar.usr_registro.clone(),
            fe_creacion: Local::now().naive_local(),
        };

        let Some(guardado) = guardar_pago_repo(tx, pago).await? else {
            error!("No se pudo guardar el pago");
            return Err(ServiceError::ValidationError("No se pudo guardar el pago.".to_string()));
        };
        
        for detalle in guardar.detalle_pagos {

            let Some(valor_pago) = Decimal::from_f32_retain(detalle.detalle_pago_valor) else {
                return Err(ServiceError::ValidationError("No se pudo obtener el valor del pago".to_string()));
            };

            if detalle.tipo_pago_id == 2 {
                let saldo = cargar_saldo_tarjeta(tx, &guardar.per_id).await?;
                if saldo.is_empty() {
                    return Err(ServiceError::ValidationError("La tarjeta no tiene saldo.".to_string()));
                }
                let restante = saldo[0];
                
                if restante <= dec!(0) {
                    return Err(ServiceError::ValidationError("La tarjeta no tiene saldo.".to_string()));
                }

                if restante < valor_pago {
                    return Err(ServiceError::ValidationError(format!("El saldo disponible de la tarjeta es inferior, saldo restante:${}",restante)));
                }

            }
            detalle_guardar.push(DetallePago { detalle_pago_id: None, 
                                            tipo_pago_id: Some(detalle.tipo_pago_id), 
                                            pago_id: Some(guardado.clone()), 
                                            detalle_pago_valor: valor_pago, 
                                            estado: Estados::Activo.to_string(), 
                                            usr_creacion: guardar.usr_registro.clone(), 
                                            fe_creacion: Local::now().naive_local(), 
                                            usr_modificacion: None, 
                                            fe_modificacion: None 
                                            });
        }

        let guardado_det = guardar_detalle(tx, detalle_guardar).await?;

        if guardado_det > 0 {
            return Ok(Response {
                codigo: Some(200),
                status: Some(String::from("Exito")),
                mensaje: Some(String::from("Exito")),
            });
        }

        return Err(ServiceError::ValidationError("Existio un error al guardar el detalle del pago.".to_string()));
    }).await
}
