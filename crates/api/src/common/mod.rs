use actix_web::{web::Data, web::Query, HttpRequest};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use log::error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

use background_jobs::QueueHandle;
use reqwest::Client;

use db_queries::{Crud, DbPool};
use utils::{
    claims::Claims, email::send_email, settings::structs::Settings, utils::MentionData, ApiError,
    AuthError,
};

//

pub mod card;
pub mod category;
pub mod comment;
pub mod order;
pub mod order_item;
pub mod payment;
pub mod product;
pub mod user;
//

// test
use actix_session::{CookieSession, Session};

//
#[derive(Serialize, Deserialize)]
pub struct Health {}

#[derive(Serialize, Clone)]
pub struct HealthResponse {
    pub server_name: String,
    pub server_version: String,
    pub server_status: String,
    pub server_description: Option<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct CollectionParams {
//   pub q: Option<String>,
//   pub fields: Option<String>,
//   pub sort: Option<String>,
//   pub page: Option<i64>,
//   pub limit: Option<i64>,
//   pub total_results: Option<bool>,
//   pub expand: Option<String>,
// }

// #[derive(Serialize, Clone)]
// pub struct CollectionResponse {
//   pub counts: i64,
//   pub has_more: bool,
//   pub offset: i64,
//   pub limit: i64,
//   pub items: Vec<RoleView>,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SingularRequest {
    pub fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CollectionRequest {
    pub fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
    pub q: Option<String>,
    pub sort: Option<Vec<String>>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub total_count: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]

pub struct GRequest {
    pub fields: Option<String>,
    pub expand: Option<String>,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionResponce<T> {
    pub items: Vec<T>,
    pub count: Option<u32>,
    pub total_results: Option<u32>,
    pub has_more: Option<bool>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

pub struct AuthContext {
    pub pool: Option<DbPool>,
    pub client: Client,
}
impl Default for GRequest {
    fn default() -> Self {
        Self {
            fields: Some(String::from("*")),
            expand: None,
            path: None,
        }
    }
}
impl GRequest {
    pub fn from_request(_conn: &PgConnection, s: &HttpRequest) -> Self {
        let query = Query::<HashMap<String, String>>::from_query(s.query_string()).unwrap();
        let mut l_return = Self::default();

        if query.get("fields").is_some() {
            l_return.fields = Some(query.get("fields").unwrap().to_string());
        }
        if query.get("path").is_some() {
            l_return.path = Some(s.match_info().query("param").to_string());
        }
        l_return
    }
}
impl AuthContext {
    pub fn create(pool: Option<DbPool>, client: Client) -> AuthContext {
        AuthContext { pool, client }
    }
    pub fn pool(&self) -> &Option<DbPool> {
        &self.pool
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
}

// get PgConnection from HttpRequest
pub fn connection(req: &HttpRequest) -> PooledConnection<ConnectionManager<PgConnection>> {
    let auth_context: Option<&Data<AuthContext>> = req.app_data::<Data<AuthContext>>();

    let v = auth_context.unwrap().as_ref().pool().clone().unwrap();
    let conn = v.get().unwrap();
    conn
}

impl Clone for AuthContext {
    fn clone(&self) -> Self {
        AuthContext {
            pool: self.pool.clone(),
            client: self.client.clone(),
        }
    }
}

pub async fn blocking<F, T>(pool: &Option<DbPool>, f: F) -> Result<T, AuthError>
where
    F: FnOnce(&diesel::PgConnection) -> T + Send + 'static,
    T: Send + 'static,
{
    let pool = pool.clone();
    let res = actix_web::web::block(move || {
        let conn = pool.unwrap().get()?;
        let res = (f)(&conn);
        Ok(res) as Result<_, AuthError>
    })
    .await?;

    res
}

// pub async fn get_user_view_from_jwt(
//     jwt: &str,
//     pool: &DbPool,
//   ) -> Result<UserView, AuthError> {
//     let claims = match Claims::decode(&jwt) {
//       Ok(claims) => claims.claims,
//       Err(_e) => return Err(ApiError::err("not_logged_in").into()),
//     };
//     let user_id = UserId(claims.sub);
//     let user_view =
//       blocking(pool, move |conn| UserView::read(conn, user_id)).await??;
//     // Check for a campus ban
//     if user_view.person.banned {
//       return Err(ApiError::err("campus_ban").into());
//     }

//     check_validator_time(&user_view.user.validator_time, &claims)?;

//     Ok(user_view)
// }

/// Checks if users's token was issued before users's password reset.
pub fn check_validator_time(
    validator_time: &chrono::NaiveDateTime,
    claims: &Claims,
) -> Result<(), AuthError> {
    let user_validation_time = validator_time.timestamp();
    if user_validation_time > claims.iat {
        Err(ApiError::err("not_logged_in").into())
    } else {
        Ok(())
    }
}

/// Checks the password length
pub fn check_password_validation(pass: &str) -> Result<(), AuthError> {
    if pass.len() > 60 {
        Err(ApiError::err("invalid_password").into())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
