use std::future::{ready, Ready};

use actix_web::{
  body::EitherBody,
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  Error, HttpResponse,
};
use futures_util::{future::LocalBoxFuture, FutureExt, TryFutureExt};
use jsonwebtoken::{decode, DecodingKey, Algorithm, Validation};
use std::env;
use crate::models::Claims;

pub struct JwtCheck;

impl<S, B> Transform<S, ServiceRequest> for JwtCheck
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtCheckMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(JwtCheckMiddleware { service }))
    }
}


pub struct JwtCheckMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for JwtCheckMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let headers = req.headers();
    let auth_header = headers.get("Authorization");
    match auth_header {
      Some(header_value) => {
        if let Ok(token) = header_value.to_str() {

          let token_secret = match env::var("TOKEN_SECRET") {
            Ok(secret) => secret,
            Err(_) => return Box::pin(async {
              Ok(req.into_response(
                HttpResponse::Unauthorized()
                  .finish()
                  .map_into_right_body(),
              ))
            }),
          };

          let validation = Validation::new(Algorithm::HS256);
          match decode::<Claims>(&token.replace("Bearer ", ""), &DecodingKey::from_secret(token_secret.as_ref()), &validation) {
            Ok(_) => {
              return self.service
                .call(req)
                .map_ok(ServiceResponse::map_into_left_body)
                .boxed_local()
            },
            Err(_) => {
              return Box::pin(async {
                Ok(req.into_response(
                  HttpResponse::Unauthorized()
                    .finish()
                    .map_into_right_body(),
                ))
              })
            }
          }
        }
      },
      None => {
        return Box::pin(async {
          Ok(req.into_response(
            HttpResponse::Unauthorized()
              .finish()
              .map_into_right_body(),
          ))
        });
      }
    }

    self.service
      .call(req)
      .map_ok(ServiceResponse::map_into_left_body)
      .boxed_local()
  }
}