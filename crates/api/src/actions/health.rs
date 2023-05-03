use crate::actions::Perform;
use crate::common::*;
use actix_web::{body::BoxBody, web::Data, Error, FromRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};
use log::info;
use utils::{settings::structs::Settings, version, AuthError, ConnectionId};

#[async_trait::async_trait(?Send)]
impl Perform for Health {
    type Response = HealthResponse;

    async fn perform(&self, context: &Data<AuthContext>) -> Result<HealthResponse, AuthError> {
        let data: &Health = &self;

        Ok(HealthResponse {
            server_name: "PetsShop APIs".to_string(),
            server_version: version::VERSION.to_string(),
            server_status: "Active".to_string(),
            server_description: Some("API Server for PetsShop".to_string()),
        })
    }
}

impl FromRequest for Health {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(
        _req: &actix_web::HttpRequest,
        _: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ok(Self {})
    }
}

impl Responder for Health {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(HealthResponse {
            server_name: "PetsShop APIs".to_string(),
            server_version: version::VERSION.to_string(),
            server_status: "Active".to_string(),
            server_description: Some("API Server for PetsShop".to_string()),
        })
    }
}
