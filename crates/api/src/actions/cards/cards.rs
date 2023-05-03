use crate::actions::ManagementTrait;
use crate::common::{card::*, connection, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::cards::cards::Card_;
use db_schema::{models::cards::*, CardId};
use db_schema::{
    models::cards::{Card, CardForm},
    naive_now,
};
use db_views::cards::cards_view::CardView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<CardRequest> for CardApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CardRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);
        let form = CardForm {
            user_id: _data.user_id.clone(),
            product_id: _data.product_id.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            CardView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CardRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_product_id = &_req.match_info().get("product_id").unwrap();
        //println!("_path_product_id = {:}", _path_product_id);
        let _product_id = CardId(_path_product_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.user_id.clone()));

        let form = CardForm {
            user_id: _data.user_id.clone(),
            product_id: _data.product_id.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response = CardView::update_item(
            &_conn,
            &_product_id,
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

        let _header_product_id = &_req.match_info().get("product_id").unwrap();

        let _product_id = CardId(_header_product_id.to_string());

        let f = CardView::delete_item(&_conn, &_product_id).await?;

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

        let _path_product_id = &_req.match_info().get("product_id").unwrap();

        let _product_id = CardId(_path_product_id.to_string());

        let _response =
            CardView::get_item(&_conn, &_product_id, &_single.fields, &_single.expand).await?;

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

        let _response = CardView::get_collection(
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
