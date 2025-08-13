use log::warn;
use mongodb::{
    bson::{doc, uuid::Error}, options::{ClientOptions, Credential, ServerAddress}, results::{InsertOneResult, UpdateResult}, Client, Collection};
use dotenvy::dotenv;

use crate::{models::data_model::token_model::Token, utils::enums::estados_enum::Estados};
#[derive(Clone)]
pub struct TokenRepo {
    col: Collection<Token>,
}

impl TokenRepo {

    pub async fn init() -> Self {
        dotenv().ok();
        /*let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(err) =>{
                error!("Existio un error al leer la variable MONGOURI {}",err);
                format!("Error cargando las variables de sesion")
            } 
        };*/
        let servidor = ServerAddress::Tcp { host: "89.117.63.107".to_string(), port: Some(4700) };
        let credenciales = Credential::builder()
                                       .username(Some("admongo".to_string()))
                                       .password(Some("holamonica".to_string()))
                                       .source(Some("admin".to_string()))
                                       .build();
        let opciones_cliente = ClientOptions::builder()
                                                                     .hosts(vec![servidor])
                                                                     .app_name(Some("tokens_tarjetas_refrigerio_rs".to_string()))
                                                                     .max_pool_size(Some(80))
                                                                     .min_pool_size(Some(10))
                                                                     .credential(Some(credenciales))
                                                                     .build();
        //let client = Client::with_uri_str(uri).await.expect("Existio un error al generar el cliente");
        let client = Client::with_options(opciones_cliente).expect("Existio un error al generar el cliente de mongo");
        let db = client.database("tokens_refrigerio");
        let col: Collection<Token> = db.collection("tokens");
        TokenRepo { col }
    }

    pub async fn guardar_token(&self, token: Token) -> Result<InsertOneResult, Error> {
        let token = self
            .col
            .insert_one(token)
            .await
            .expect("Error al registrar el token");
        Ok(token)
    }

    pub async fn buscar_token(&self, token: &str) -> Option<Token> {
        let filter = doc! {"token": token};
        let token_detail = self
            .col
            .find_one(filter)
            .await;
        match token_detail {
            Ok(resp)=>resp,
            Err(error) =>{
                warn!("Existio un problema al buscar el token {}",error);
                None
            }
        }
    }

    pub async fn borrar_token(&self, token: &str)->Result<UpdateResult, mongodb::error::Error> {
        let filter = doc! {"token": token};
        let new_doc = doc! {
            "$set":
                {
                    "estado": Estados::Eliminado.to_string()
                },
        };
        self
            .col
            .update_many(filter, new_doc)
            .await        
    }

}
