use crate::{
  settings::structs::{RateLimitConfig, Settings},
  utils::get_ip,
  AuthError, IpAddr,
};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use rate_limiter::{RateLimitType, RateLimiter};
use std::{
  future::Future,
  sync::Arc,
  task::{Context, Poll},
};
use tokio::sync::Mutex;

pub mod rate_limiter;

#[derive(Debug, Clone)]
pub struct RateLimit {
  // it might be reasonable to use a std::sync::Mutex here, since we don't need to lock this
  // across await points
  pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

#[derive(Debug, Clone)]
pub struct RateLimited {
  rate_limiter: Arc<Mutex<RateLimiter>>,
  type_: RateLimitType,
}

pub struct RateLimitedMiddleware<S> {
  rate_limited: RateLimited,
  service: S,
}

impl RateLimit {
  pub fn signup(&self) -> RateLimited {
    self.kind(RateLimitType::Signup)
  }
  pub fn signin(&self) -> RateLimited {
    self.kind(RateLimitType::Signin)
  }
  pub fn invite(&self) -> RateLimited {
    self.kind(RateLimitType::Invite)
  }

  pub fn settings(&self) -> RateLimited {
    self.kind(RateLimitType::Settings)
  }

  pub fn recovery(&self) -> RateLimited {
    self.kind(RateLimitType::Recovery)
  }

  pub fn verify(&self) -> RateLimited {
    self.kind(RateLimitType::Verify)
  }

  pub fn otp(&self) -> RateLimited {
    self.kind(RateLimitType::Otp)
  }

  pub fn user(&self) -> RateLimited {
    self.kind(RateLimitType::User)
  }

  pub fn signout(&self) -> RateLimited {
    self.kind(RateLimitType::SignOut)
  }

  pub fn authorize(&self) -> RateLimited {
    self.kind(RateLimitType::Authorize)
  }

  pub fn callback(&self) -> RateLimited {
    self.kind(RateLimitType::Callback)
  }

  pub fn audit(&self) -> RateLimited {
    self.kind(RateLimitType::Audit)
  }

  pub fn users(&self) -> RateLimited {
    self.kind(RateLimitType::Users)
  }

  pub fn generatelink(&self) -> RateLimited {
    self.kind(RateLimitType::GenerateLink)
  }

  pub fn saml(&self) -> RateLimited {
    self.kind(RateLimitType::Saml)
  }

  pub fn tenants(&self) -> RateLimited {
    self.kind(RateLimitType::Tenants)
  }

  fn kind(&self, type_: RateLimitType) -> RateLimited {
    RateLimited {
      rate_limiter: self.rate_limiter.clone(),
      type_,
    }
  }
}

impl RateLimited {
  pub async fn wrap<T, E>(
    self,
    ip_addr: IpAddr,
    fut: impl Future<Output = Result<T, E>>,
  ) -> Result<T, E>
  where
    E: From<AuthError>,
  {
    // Does not need to be blocking because the RwLock in settings never held across await points,
    // and the operation here locks only long enough to clone
    let rate_limit: RateLimitConfig = Settings::get().rate_limit();

    // before
    {
      let mut limiter = self.rate_limiter.lock().await;

      match self.type_ {
        RateLimitType::Signup => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.signups,
            rate_limit.signups_per_second,
            false,
          )?;

          drop(limiter);
          return fut.await;
        }
        RateLimitType::Signin => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.signins,
            rate_limit.signins_per_second,
            false,
          )?;

          drop(limiter);
          return fut.await;
        }
        RateLimitType::Invite => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.invites,
            rate_limit.invites_per_second,
            true,
          )?;
        }
        RateLimitType::Settings => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.settings,
            rate_limit.settings_per_second,
            true,
          )?;
        }
        RateLimitType::Recovery => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.recoveries,
            rate_limit.recoveries_per_second,
            false,
          )?;
        }
        RateLimitType::Verify => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.verify,
            rate_limit.verify_per_second,
            false,
          )?;
        }
        RateLimitType::Otp => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.otp,
            rate_limit.otp_per_second,
            true,
          )?;
        }
        RateLimitType::Token => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.token,
            rate_limit.token_per_second,
            true,
          )?;
        }
        RateLimitType::User => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.user,
            rate_limit.user_per_second,
            true,
          )?;
        }
        RateLimitType::SignOut => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.signout,
            rate_limit.signout_per_second,
            true,
          )?;
        }
        RateLimitType::Authorize => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.authorize,
            rate_limit.authorize_per_second,
            true,
          )?;
        }
        RateLimitType::Callback => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.callback,
            rate_limit.callback_per_second,
            true,
          )?;
        }
        RateLimitType::Audit => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.audit,
            rate_limit.audit_per_second,
            true,
          )?;
        }
        RateLimitType::Users => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.users,
            rate_limit.users_per_second,
            true,
          )?;
        }
        RateLimitType::GenerateLink => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.generate_link,
            rate_limit.generate_link_per_second,
            true,
          )?;
        }
        RateLimitType::Saml => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.saml,
            rate_limit.saml_per_second,
            true,
          )?;
        }
        RateLimitType::Tenants => {
          limiter.check_rate_limit_full(
            self.type_,
            &ip_addr,
            rate_limit.tenants,
            rate_limit.tenants_per_second,
            true,
          )?;
        }
      };
    }

    let res = fut.await;

    // after
    {
      let mut limiter = self.rate_limiter.lock().await;
      if res.is_ok() {
        match self.type_ {
          RateLimitType::Signup => {
            limiter.check_rate_limit_full(
              self.type_,
              &ip_addr,
              rate_limit.signups,
              rate_limit.signups_per_second,
              false,
            )?;
          }
          RateLimitType::Invite => {
            limiter.check_rate_limit_full(
              self.type_,
              &ip_addr,
              rate_limit.invites,
              rate_limit.invites_per_second,
              false,
            )?;
          }
          _ => (),
        };
      }
    }

    res
  }
}

impl<S, B> Transform<S, ServiceRequest> for RateLimited
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type InitError = ();
  type Transform = RateLimitedMiddleware<S>;
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ready(Ok(RateLimitedMiddleware {
      rate_limited: self.clone(),
      service,
    }))
  }
}

impl<S, B> Service<ServiceRequest> for RateLimitedMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
  //forward_ready!(service);
  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let ip_addr = get_ip(&req.connection_info());

    let fut = self
      .rate_limited
      .clone()
      .wrap(ip_addr, self.service.call(req));

    Box::pin(async move {
      let res = fut.await?;
      Ok(res)
    })
  }
}
