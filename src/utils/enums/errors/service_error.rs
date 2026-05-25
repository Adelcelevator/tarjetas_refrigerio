use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use log::error;
use crate::models::responses::response::Response;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("{0}")]
    BdError(String),
    #[error("Busqueda vacia en la base de datos: {0}")]
    NotFound(String),
    #[error("Token vacio: {0}")]
    MissingToken(String),
    #[error("Token no valido: {0}")]
    InvalidToken(String),
    #[error("Existio un error al validar: {0}")]
    ValidationError(String),
    #[error("No se pudo encontrar a la persona: {0}")]
    PersonaNoEncontrada(String),
    #[error("Error de configuración: {0}")]
    ConfiguracionError(String),
    #[error("El comprobante está vacío o es inválido.")]
    ComprobanteInvalido(String),
    #[error("El comprobante ya existe en el sistema.")]
    ComprobanteExiste,
    #[error("Existio un error al procesar el archivo.")]
    Base64DecodeError,
    #[error("Existio un error interno del sistema.")]
    InternalServerError,
}

impl ResponseError for ServiceError {

    fn error_response(&self)->HttpResponse{
        let error = self.to_string();
        error!("Existio un error al procesar la peticion: {}",error);
        let respuesta = Response {
                            codigo: Some(500),
                            status: Some("Error".to_string()),
                            mensaje: Some(error),
                        };
        
        HttpResponse::Ok().json(respuesta)
    }

}