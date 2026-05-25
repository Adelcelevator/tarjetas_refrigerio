use diesel::{ExpressionMethods, JoinOnDsl, PgTextExpressionMethods, QueryDsl, alias};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use log::error;

use crate::{models::data_model::postgres::persona_model::{BuscarPersonaCobro, PersonaCobroConsulta, PersonaConsulta}, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

use super::db_tarjetas_repository::db_tarjetas::{tbl_persona, tbl_usuario, tbl_unidades};
pub async fn buscar_persona(con:&mut AsyncPgConnection, id: &i32)->Result<PersonaConsulta, ServiceError>{
    let search = tbl_persona::dsl::tbl_persona
            .filter(tbl_persona::per_id.eq(*id))
            .first::<PersonaConsulta>(con).await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el comprobante por numero persona: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los comprobantes.".to_string()))
        }
    }
}

pub async fn buscar_persona_por_usuario(con:&mut AsyncPgConnection, usuario: &String)->Result<PersonaConsulta, ServiceError>{
    let (persona, 
         usu) = alias!(tbl_persona as persona,
                          tbl_usuario as usuario);
    let search = persona.inner_join(usu)
                                 .filter(usu.field(tbl_usuario::usu_usuario).eq(usuario))
                                 .select(persona.fields(tbl_persona::all_columns))
                                 .first::<PersonaConsulta>(con).await;
    
    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el comprobante por numero persona: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar los comprobantes.".to_string()))
        }
    }
}

pub async fn buscar_persona_para_cobro(con:&mut AsyncPgConnection, 
                                 persona_buscada:BuscarPersonaCobro)->Result<Vec<PersonaCobroConsulta>,ServiceError>{
    
    let identi_bus = persona_buscada.per_identificacion.is_some();
    let nombre_bus = persona_buscada.per_nombre.is_some();
    let unidad_bus = persona_buscada.per_unidad.is_some();
    
    if identi_bus && nombre_bus && unidad_bus {

        let Some(unidad_nombre) =persona_buscada.per_unidad else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let Some(per_nombre) = persona_buscada.per_nombre else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let Some(identificacion) = persona_buscada.per_identificacion else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };


        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(unidad_nombre))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",per_nombre)))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",identificacion)))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }

    if nombre_bus && unidad_bus {

        let Some(unidad_nombre) =persona_buscada.per_unidad else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let Some(per_nombre) = persona_buscada.per_nombre else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(unidad_nombre))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",per_nombre)))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }

    if identi_bus && unidad_bus {

        let Some(unidad_nombre) =persona_buscada.per_unidad else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let Some(identificacion) = persona_buscada.per_identificacion else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(unidad_nombre))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",identificacion)))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }

    if identi_bus && nombre_bus {

        let Some(per_nombre) = persona_buscada.per_nombre else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let Some(identificacion) = persona_buscada.per_identificacion else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };
        
        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",per_nombre)))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",identificacion)))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }
    if nombre_bus {
        
        let Some(per_nombre) = persona_buscada.per_nombre else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_persona::dsl::per_nombre.ilike(format!("%{}%",per_nombre)))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }
    if identi_bus {

        let Some(identificacion) = persona_buscada.per_identificacion else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_persona::dsl::per_identificacion.ilike(format!("%{}%",identificacion)))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }
    
    if unidad_bus {

        let Some(unidad_nombre) =persona_buscada.per_unidad else {
            return Err(ServiceError::ValidationError("No se pudo definir una unidad.".to_string()));
        };

        let buscando = tbl_persona::dsl::tbl_persona
                       .inner_join(tbl_unidades::dsl::tbl_unidades.on(tbl_persona::dsl::unidad_id.eq(tbl_unidades::dsl::unidad_id)))
                       .filter(tbl_unidades::dsl::unidad_nombre.eq(unidad_nombre))
                       .filter(tbl_persona::dsl::estado.eq(Estados::Activo.to_string()))
                       .select((tbl_persona::per_id,tbl_persona::per_nombre, tbl_unidades::unidad_nombre))
                       .load::<PersonaCobroConsulta>(con).await;
        let buscado = match buscando {
                Ok(encontrado)=> encontrado,
                Err(error) =>{
                    error!("Existio un error al realizar la busqueda de la persona: {}",error);
                    vec![]
                }
        };
        return Ok(buscado);
    }

    Ok(vec![])
}