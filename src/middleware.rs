use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
    web
};
use futures_util::future::LocalBoxFuture;
use crate::persistance::accounts::is_good_password_account;
use mysql::Pool;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct SayHi {
  db: web::Data<Pool>,
}

impl SayHi {
  pub fn new(db: web::Data<Pool>) -> Self {
      SayHi { db }
  }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for SayHi
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      let db = self.db.clone();
        ready(Ok(SayHiMiddleware { service, db }))
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
    db: web::Data<Pool>,
}

impl<S, B> Service<ServiceRequest> for SayHiMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest,) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        match is_good_password_account(&self.db, String::from("test"), String::from("test")) {
          Ok(result) => println!("Response: {}", result),
          Err(_) => println!("Error")
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}