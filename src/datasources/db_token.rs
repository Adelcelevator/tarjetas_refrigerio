use mongodb::Client;
use log::error;

use crate::utils::env_utils::get_variable;

#[derive(Clone)]
pub struct TokenRepo;

impl TokenRepo {

    pub async fn init()->Client {
        let Some(url) = get_variable::<String>("MONGOURI") else {
            error!("Existio un error al cargar la variable DATABASE_URL");
            std::process::exit(1);
        };
        let cliente_req = Client::with_uri_str(url).await;

        match cliente_req {
            Ok(cl)=>cl,
            Err(error)=>{
                error!("Exisitio un error al generar el cliente: {}",error);
                std::process::exit(1);
            }
        }
    }
}