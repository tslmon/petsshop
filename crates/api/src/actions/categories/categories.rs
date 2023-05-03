use crate::actions::ManagementTrait;
use crate::common::{category::*, connection, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::categories::categories::Category_;
use db_schema::{models::categories::*, CategoryId};
use db_schema::{
    models::categories::{Category, CategoryForm},
    naive_now,
};
use db_views::categories::categories_view::CategoryView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

#[async_trait::async_trait(?Send)]
impl ManagementTrait<CategoryRequest> for CategoryApi {
    type Response = HttpResponse;

    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CategoryRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);
        //println!("{:#?}", _data);
        let form = CategoryForm {
            name: _data.name.clone(),
            parent: _data.parent.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            CategoryView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<CategoryRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_category_id = &_req.match_info().get("category_id").unwrap();
        //println!("_path_category_id = {:}", _path_category_id);
        let _category_id = CategoryId(_path_category_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.name.clone()));

        let form = CategoryForm {
            name: _data.name.clone(),
            parent: _data.parent.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response = CategoryView::update_item(
            &_conn,
            &_category_id,
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

        let _header_category_id = &_req.match_info().get("category_id").unwrap();

        let _category_id = CategoryId(_header_category_id.to_string());

        let f = CategoryView::delete_item(&_conn, &_category_id).await?;

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

        let _path_category_id = &_req.match_info().get("category_id").unwrap();

        let _category_id = CategoryId(_path_category_id.to_string());

        let _response =
            CategoryView::get_item(&_conn, &_category_id, &_single.fields, &_single.expand).await?;

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

        let _response = CategoryView::get_collection(
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
