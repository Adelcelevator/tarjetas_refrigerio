use mongodb::{Client, bson::doc, results::{InsertOneResult, UpdateResult}};
use log::error;

use crate::{models::data_model::mongo::token_model::Token, utils::enums::{errors::service_error::ServiceError, estados_enum::Estados}};

pub async fn guardar_token(cliente: Client, token: Token) ->Result<InsertOneResult, ServiceError> {
    let col = cliente.database("tarjetas_tokens").collection::<Token>("session_tokens");
    let insert = col.insert_one( token).await;
    
    match insert{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al guardar el token: {}",error);
            Err(ServiceError::BdError("Existio un error al guardar el token.".to_string()))
        }
    }
}

pub async fn buscar_token(cliente: Client, token: &str) ->Result<Option<Token>, ServiceError> {
    let col = cliente.database("tarjetas_tokens").collection::<Token>("session_tokens");
    let filter = doc! {"token": token};
    let search = col
                        .find_one(filter)
                        .await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al buscar el token: {}",error);
            Err(ServiceError::BdError("Existio un error al buscar el token.".to_string()))
        }
    }
}

pub async fn borrar_token(cliente: Client, token: &str)->Result<UpdateResult, ServiceError> {
    let col = cliente.database("tarjetas_tokens").collection::<Token>("session_tokens");
    let filter = doc! {"token": token};
    let new_doc = doc! {
        "$set":
            {
                "estado": Estados::Eliminado.to_string()
            },
    };
    let search = col
                    .update_many(filter, new_doc)
                    .await;

    match search{
        Ok(res)=>Ok(res),
        Err(error)=>{
            error!("Existio un error al borrar el token: {}",error);
            Err(ServiceError::BdError("Existio un error al borrar el token.".to_string()))
        }
    }
}