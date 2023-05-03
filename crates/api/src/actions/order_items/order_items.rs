use crate::actions::ManagementTrait;
use crate::common::{connection, order_item::*, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::order_items::order_items::OrderItem_;
use db_schema::{models::order_items::*, OrderItemId};
use db_schema::{
    models::order_items::{OrderItem, OrderItemForm},
    naive_now,
};
use db_views::order_items::order_items_view::OrderItemView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<OrderItemRequest> for OrderItemApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<OrderItemRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);
        let form = OrderItemForm {
            order_id: _data.order_id.clone(),
            product_id: _data.product_id.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            OrderItemView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<OrderItemRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_order_item_id = &_req.match_info().get("order_item_id").unwrap();
        //println!("_path_order_item_id = {:}", _path_order_item_id);
        let _order_item_id = OrderItemId(_path_order_item_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.order_id.clone()));

        let form = OrderItemForm {
            order_id: _data.order_id.clone(),
            product_id: _data.product_id.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response = OrderItemView::update_item(
            &_conn,
            &_order_item_id,
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

        let _header_order_item_id = &_req.match_info().get("order_item_id").unwrap();

        let _order_item_id = OrderItemId(_header_order_item_id.to_string());

        let f = OrderItemView::delete_item(&_conn, &_order_item_id).await?;

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

        let _path_order_item_id = &_req.match_info().get("order_item_id").unwrap();

        let _order_item_id = OrderItemId(_path_order_item_id.to_string());

        let _response =
            OrderItemView::get_item(&_conn, &_order_item_id, &_single.fields, &_single.expand)
                .await?;

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

        let _response = OrderItemView::get_collection(
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
