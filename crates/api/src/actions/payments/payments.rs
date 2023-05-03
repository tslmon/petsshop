use crate::actions::ManagementTrait;
use crate::common::{connection, payment::*, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::payments::payments::Payment_;
use db_schema::{models::payments::*, PaymentId};
use db_schema::{
    models::payments::{Payment, PaymentForm},
    naive_now,
};
use db_views::payments::payments_view::PaymentView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<PaymentRequest> for PaymentApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<PaymentRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);
        let form = PaymentForm {
            order_id: _data.order_id.clone(),
            amount: _data.amount.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            PaymentView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<PaymentRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_payment_id = &_req.match_info().get("payment_id").unwrap();
        //println!("_path_payment_id = {:}", _path_payment_id);
        let _payment_id = PaymentId(_path_payment_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.amount.clone()));

        let form = PaymentForm {
            order_id: _data.order_id.clone(),
            amount: _data.amount.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response = PaymentView::update_item(
            &_conn,
            &_payment_id,
            &form,
            &_single.fields,
            &_single.expand,
        )
        .await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn delete_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _header_payment_id = &_req.match_info().get("payment_id").unwrap();

        let _payment_id = PaymentId(_header_payment_id.to_string());

        let f = PaymentView::delete_item(&_conn, &_payment_id).await?;

        if f == 1 {
            Ok(HttpResponse::NoContent().finish())
        } else {
            Ok(HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("NotFound"))
        }
    }

    async fn get_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_payment_id = &_req.match_info().get("payment_id").unwrap();

        let _payment_id = PaymentId(_path_payment_id.to_string());

        let _response =
            PaymentView::get_item(&_conn, &_payment_id, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn get_collection(
        _req: HttpRequest,
        _coll: CollectionRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _response = PaymentView::get_collection(
            &_conn,
            &_coll.fields,
            &_coll.expand,
            &_coll.q,
            &_coll.sort,
            &_coll.offset,
            &_coll.limit,
            &_coll.total_count,
        )
        .await?;

        let _body = serde_json::to_string(&_response)?;

        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }
}
