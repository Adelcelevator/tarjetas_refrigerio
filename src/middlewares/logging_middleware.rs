use std::sync::Arc;

use actix_web::{Error, dev::{Service, ServiceRequest, ServiceResponse, Transform}};
use futures::future::{LocalBoxFuture, Ready, ok};
use log::error;
use tracing::{Instrument, info_span};
use uuid::Uuid;

pub struct LogingStruct;

impl< S , B > Transform< S , ServiceRequest > for LogingStruct 
                                              where S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
                                                    S::Future: 'static,
                                                    B: 'static, {
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LogingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform,Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LogingMiddleware{service:Arc::new(service),})
    }
}

pub struct LogingMiddleware<S>{
    service: Arc<S>,
}

impl<S,B> Service<ServiceRequest> for LogingMiddleware<S> 
                                  where
                                  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
                                  S::Future: 'static,
                                  B: 'static,{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static,Result<Self::Response,Self::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        
        let span = info_span!(
            "request",
            id = %Uuid::new_v4().to_string()
        );

        let future_call = self.service.call(req);
        Box::pin( async move {
            future_call.await.map_err(|error| {
                error!("Exisito un error en el middleware: {}", error);
                actix_web::error::ErrorInternalServerError("Existio un error a la interna del servidor")
            })
        }.instrument(span))
    }
}