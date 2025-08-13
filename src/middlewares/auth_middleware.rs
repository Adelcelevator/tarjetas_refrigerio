use std::sync::Arc;

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform},Error};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::error;

use crate::{repository::token_repo::TokenRepo, utils::{enums::estados_enum::Estados, token_user::validar_token}};

pub struct MiddleAuthentication{
    token_repo:Arc<TokenRepo>,
}

impl MiddleAuthentication {
    pub fn new(token_repo:Arc<TokenRepo>)->Self {
        MiddleAuthentication{
            token_repo: token_repo
        }
    }
}


impl< S , B > Transform< S , ServiceRequest > for MiddleAuthentication 
    where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
          S::Future: 'static,
          B: 'static, 
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform,Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware{service: Arc::new(service),
                                    token_repo: Arc::clone(&self.token_repo)
                                })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Arc<S>,
    token_repo: Arc<TokenRepo>,
}

impl<S,B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
 {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static,Result<Self::Response,Self::Error>>;
 
    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }
 
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token_repo = Arc::clone(&self.token_repo);
        let cookie_bus  = req.cookie("token");
        let path = String::from(req.path());
        if path.eq("/tarjetasRefrigerio/user/login") {
            return Box::pin(self.service.call(req)); // Permite pasar sin validación
        }
        let future_call = self.service.call(req);
        Box::pin( async move {
            let mut cookie_value = String::new();
            if cookie_bus.is_some(){
                let ck = cookie_bus.unwrap();
                cookie_value = ck.value().to_string();
            }
            if !cookie_value.is_empty() {
                let codificado = urlencoding::encode(&cookie_value);
                let bus = token_repo.buscar_token(&codificado).await;
                if bus.is_some() {
                    let token = bus.unwrap();
                    if token.estado == Estados::Activo {
                        let claims = validar_token(&codificado);
                        if !claims.roles.is_empty() {
                            /* TODO MANEJO DE ROLES BACK
                            let mut fin:bool;
                            for rol in claims.roles{
                                fin = match path.as_str() {
                                    "/tarjetasRefrigerio/comprobante/autorizar" => rol.eq(Roles::AprobacionComprobantes.to_string().as_str()),
                                    "/tarjetasRefrigerio/comprobante/guardar" => rol.eq(Roles::RegistrarComprobante.to_string().as_str()),
                                    _ => false
                                };
                                if fin {
                                    break;
                                }
                            }
                            */
                            let call_service = future_call.await;
                            let result = match call_service {
                                Ok(res) => Some(res),
                                Err(error) =>{
                                    error!("Exisito un error en el middleware: {}",error);
                                    None
                                }
                            };
                            if result.is_some(){
                                Ok(result.unwrap())
                            }else{
                                Err(actix_web::error::ErrorInternalServerError("Existio un error a la interna del servidor"))
                            }
                        }else{
                            Err(actix_web::error::ErrorUnauthorized("Token No Valido"))
                        }
                    }else{
                        Err(actix_web::error::ErrorUnauthorized("Estado del Token No Valido"))
                    }
                }else{
                    Err(actix_web::error::ErrorUnauthorized("Token No encontrado"))
                }
            }else{
                Err(actix_web::error::ErrorUnauthorized("Token Vacio"))
            }
        })
    }
 }
