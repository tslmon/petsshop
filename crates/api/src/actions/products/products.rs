use crate::actions::ManagementTrait;
use crate::common::{connection, product::*, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::products::products::Product_;
use db_schema::{models::products::*, ProductId};
use db_schema::{
    models::products::{Product, ProductForm},
    naive_now,
};
use db_views::products::products_view::ProductView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<ProductRequest> for ProductApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<ProductRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);
        let form = ProductForm {
            name: _data.name.clone(),
            description: _data.description.clone(),
            image: _data.image.clone(),
            price: _data.price.clone(),
            quantity: _data.quantity.clone(),
            category_id: _data.category_id.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            ProductView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<ProductRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_product_id = &_req.match_info().get("product_id").unwrap();
        //println!("_path_product_id = {:}", _path_product_id);
        let _product_id = ProductId(_path_product_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.name.clone()));

        let form = ProductForm {
            name: _data.name.clone(),
            description: _data.description.clone(),
            image: _data.image.clone(),
            price: _data.price.clone(),
            quantity: _data.quantity.clone(),
            category_id: _data.category_id.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response = ProductView::update_item(
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

        let _product_id = ProductId(_header_product_id.to_string());

        let f = ProductView::delete_item(&_conn, &_product_id).await?;

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

        let _product_id = ProductId(_path_product_id.to_string());

        let _response =
            ProductView::get_item(&_conn, &_product_id, &_single.fields, &_single.expand).await?;

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

        let _response = ProductView::get_collection(
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
