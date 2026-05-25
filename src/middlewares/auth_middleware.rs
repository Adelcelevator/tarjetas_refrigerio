use std::sync::Arc;

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform},Error};
use futures::future::{ok, LocalBoxFuture, Ready};
use log::error;
use mongodb::Client;

use crate::{repository::mongo::token_repo::buscar_token, utils::{enums::estados_enum::Estados, token_user::validar_token}};

pub struct MiddleAuthentication{
    token_repo:Client,
}

impl MiddleAuthentication {
    pub fn new(token_repo:Client)->Self {
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
                                    token_repo: self.token_repo.clone()
                                })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Arc<S>,
    token_repo: Client,
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
        let cookie_bus  = req.cookie("token");
        let path = req.path();

        if path == "/tarjetasRefrigerio/user/login" {
            return Box::pin(self.service.call(req)); // Permite pasar sin validación
        }

        let future_call = self.service.call(req);
        let token_repo = self.token_repo.clone();

        Box::pin( async move {
            // 1. Validar presencia de cookie
            let cookie = cookie_bus.ok_or_else(|| actix_web::error::ErrorUnauthorized("Token Vacio"))?;
            let cookie_value = cookie.value();
            if cookie_value.is_empty() {
                return Err(actix_web::error::ErrorUnauthorized("Token Vacio"));
            }

            let codificado = urlencoding::encode(cookie_value).to_string();

            // 2. Validar existencia y estado del token en base de datos
            let tk = match buscar_token(token_repo, &codificado).await{
                            Ok(res)=>res,
                            Err(error)=>{
                                return Err(actix_web::error::ErrorUnauthorized(format!("Exisitio un error al buscar el token: {}",error)));
                            }
                        };
            let Some(token) = tk else {
                return Err(actix_web::error::ErrorUnauthorized("Token No encontrado"));
            };
            if token.estado != Estados::Activo {
                return Err(actix_web::error::ErrorUnauthorized("Estado del Token No Valido"));
            }

            // 3. Validar claims del JWT
            let claims = validar_token(&codificado)?;
            if claims.roles.is_empty() {
                return Err(actix_web::error::ErrorUnauthorized("Token No Valido"));
            }

            // 4. Validar roles según el path (Implementación de seguridad por ruta)
            /*let autorizado = match path {
                "/tarjetasRefrigerio/comprobante/autorizar" => {
                    claims.roles.iter().any(|r| r == "AprobacionComprobantes")
                }
                "/tarjetasRefrigerio/comprobante/guardar" => {
                    claims.roles.iter().any(|r| r == "RegistrarComprobante")
                }
                _ => true, // Otros paths protegidos solo requieren un token válido
            };

            if !autorizado {
                return Err(actix_web::error::ErrorForbidden("No tiene permisos para realizar esta acción"));
            }*/

            // 5. Ejecutar el servicio si todas las guardas pasaron
            future_call.await.map_err(|error| {
                error!("Exisito un error en el middleware: {}", error);
                actix_web::error::ErrorInternalServerError("Existio un error a la interna del servidor")
            })
        })
    }
 }
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
