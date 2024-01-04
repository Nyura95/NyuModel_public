use std::{
  cell::RefCell,
  cmp::min,
  future::{ready, Ready},
};

use actix_web::{
  body::EitherBody,
  dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
  Error, HttpResponse,
};
use chrono::{Local, NaiveDateTime};
use futures_util::{future::LocalBoxFuture, FutureExt, TryFutureExt};

#[doc(hidden)]
pub struct RateLimitService<S> {
  service: S,
  token_bucket: RefCell<TokenBucket>,
}

impl<S, B> Service<ServiceRequest> for RateLimitService<S>
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

      if !self.token_bucket.borrow_mut().allow_query() {
          return Box::pin(async {
              Ok(req.into_response(
                  HttpResponse::TooManyRequests()
                      .finish()
                      .map_into_right_body(),
              ))
          });
      }

      self.service
          .call(req)
          .map_ok(ServiceResponse::map_into_left_body)
          .boxed_local()
  }
}

#[derive(Clone, Debug)]
pub struct RateLimit {
  limit: u64,
}

impl RateLimit {
  pub fn new(limit: u64) -> Self {
      Self { limit }
  }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimit
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Transform = RateLimitService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(RateLimitService {
          service,
          token_bucket: RefCell::new(TokenBucket::new(self.limit)),
      }))
  }
}

struct TokenBucket {
  limit: u64,
  capacity: u64,
  last_req_time: NaiveDateTime,
  tokens: u64,
}

impl TokenBucket {
  fn new(limit: u64) -> Self {
      TokenBucket {
          limit,
          last_req_time: NaiveDateTime::UNIX_EPOCH,
          capacity: limit,
          tokens: 0,
      }
  }
  fn allow_query(&mut self) -> bool {
      let current_time = Local::now().naive_local();

      let time_elapsed = (current_time.timestamp() - self.last_req_time.timestamp()) as u64;

      let tokens_to_add = time_elapsed * self.limit / 10;

      self.tokens = min(self.tokens + tokens_to_add, self.capacity);

      if self.tokens > 0 {
          self.last_req_time = current_time;
          self.tokens -= 1;
          true
      } else {
          false
      }
  }
}