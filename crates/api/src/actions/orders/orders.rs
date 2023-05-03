use crate::actions::ManagementTrait;
use crate::common::{connection, order::*, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::orders::orders::Order_;
use db_schema::{models::orders::*, OrderId};
use db_schema::{
    models::orders::{Order, OrderForm},
    naive_now,
};
use db_views::orders::orders_view::OrderView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<OrderRequest> for OrderApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<OrderRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);
        let form = OrderForm {
            user_id: Some(_data.user_id.clone()),
            status: Some("pending".to_string()),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };
        println!("asd");

        let _response =
            OrderView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<OrderRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_order_id = &_req.match_info().get("order_id").unwrap();
        //println!("_path_order_id = {:}", _path_order_id);
        let _order_id = OrderId(_path_order_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.user_id.clone()));

        let form = OrderForm {
            user_id: Some(_data.user_id.clone()),
            status: Some(_data.status.clone()),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            OrderView::update_item(&_conn, &_order_id, &form, &_single.fields, &_single.expand)
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

        let _header_order_id = &_req.match_info().get("order_id").unwrap();

        let _order_id = OrderId(_header_order_id.to_string());

        let f = OrderView::delete_item(&_conn, &_order_id).await?;

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

        let _path_order_id = &_req.match_info().get("order_id").unwrap();

        let _order_id = OrderId(_path_order_id.to_string());

        let _response =
            OrderView::get_item(&_conn, &_order_id, &_single.fields, &_single.expand).await?;

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

        let _response = OrderView::get_collection(
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

