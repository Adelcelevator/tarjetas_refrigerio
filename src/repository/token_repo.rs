use std::env;
use mongodb::{
    bson::{doc},
    Collection, Client, results::{InsertOneResult, UpdateResult}, bson::uuid::Error};
use dotenv::dotenv;
use crate::models::token_model::Token;
pub struct TokenRepo {
    col: Collection<Token>,
}

impl TokenRepo {

    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error cargando las variables de sesion"),
        };
        let client = Client::with_uri_str(uri).await.expect("Existio un error al generar el cliente");
        let db = client.database("idp_sso");
        let col: Collection<Token> = db.collection("tokens");
        TokenRepo { col }
    }

    pub async fn guardar_token(&self, token: Token) -> Result<InsertOneResult, Error> {
        let new_doc = Token {
            id: None,
            usuario: token.usuario,
            estado: token.estado,
            fecha_registro: token.fecha_registro,
            token: token.token,
        };
        let token = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error al registrar el token");
        Ok(token)
    }

    pub async fn buscar_token(&self, token: &String) -> Result<Token, Error> {
        let filter = doc! {"token": token};
        let token_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error al obtener el token");
        Ok(token_detail.expect("Error al obtener el token"))
    }

    pub async fn borrar_token(&self, token: &String) -> Result<UpdateResult, Error> {
        let filter = doc! {"token": token};
        let new_doc = doc! {
            "$set":
                {
                    "estado": "Inactivo"
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error al borrar el token");
        Ok(updated_doc)
    }

}
