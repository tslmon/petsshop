use crate::actions::ManagementTrait;
use crate::common::{connection, user::*, CollectionRequest, SingularRequest};
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web::Json,
    Error, FromRequest, HttpRequest, HttpResponse, Responder,
};
use db_queries::models::users::users::User_;
use db_schema::{models::users::*, UserId};
use db_schema::{
    models::users::{User, UserForm},
    naive_now,
};
use db_views::users::users_view::UserView;
use db_views::RequestCollection;
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::future::{ok, Ready};
use rand::{distributions::Alphanumeric, Rng};
use serde_json::Value;

// herew common deer todorhoilson struct iin dagu irwel heregjuuulelt hiinee.
#[async_trait::async_trait(?Send)]
impl ManagementTrait<UserRequest> for UserApi {
    type Response = HttpResponse;

    async fn create_item(
        // protocol n hhtp req baina.
        _req: HttpRequest,
        // 1 mur bichleg ruu handana.
        _single: SingularRequest,
        // json data awnaa.
        _data: Json<UserRequest>,
        // zuw bol response buruu bol error zaana.
    ) -> Result<Self::Response, ApiError> {
        // db tei holbogdoh.
        let _conn = connection(&_req);

        // common struct iig zadalj --> userForm struct ruu hiine --> user struct -->user schema-->table.
        //println!("{:#?}", _data);
        // extract hiij baina.
        let form = UserForm {
            fname: _data.fname.clone(),
            lname: _data.lname.clone(),
            gender: _data.gender.clone(),
            email: _data.email.clone(),
            phone_number: _data.phone_number.clone(),
            address: _data.address.clone(),
            user_name: _data.user_name.clone(),
            pwd: _data.pwd.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };
        // uildliig hiine.
        let _response =
            UserView::create_item(&_conn, &form, &_single.fields, &_single.expand).await?;

        // response iig body ruu bichne.
        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Created()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<UserRequest>,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _path_user_id = &_req.match_info().get("user_id").unwrap();
        //println!("_path_user_id = {:}", _path_user_id);
        let _user_id = UserId(_path_user_id.to_string());

        let _method = _req.method();
        println!("data = {:#?}", Some(_data.phone_number.clone()));

        let form = UserForm {
            fname: _data.fname.clone(),
            lname: _data.lname.clone(),
            gender: _data.gender.clone(),
            email: _data.email.clone(),
            phone_number: _data.phone_number.clone(),
            address: _data.address.clone(),
            user_name: _data.user_name.clone(),
            pwd: _data.pwd.clone(),
            created_by: Some("user".to_string()),
            updated_by: Some("user".to_string()),
        };

        let _response =
            UserView::update_item(&_conn, &_user_id, &form, &_single.fields, &_single.expand)
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

        let _header_user_id = &_req.match_info().get("user_id").unwrap();

        let _user_id = UserId(_header_user_id.to_string());

        let f = UserView::delete_item(&_conn, &_user_id).await?;

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

        let _path_user_id = &_req.match_info().get("user_id").unwrap();

        let _user_id = UserId(_path_user_id.to_string());

        let _response =
            UserView::get_item(&_conn, &_user_id, &_single.fields, &_single.expand).await?;

        let _body = serde_json::to_string(&_response)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(_body))
    }

    async fn get_collection(
        _req: HttpRequest,
        // olon mur bichleg
        _coll: CollectionRequest,
    ) -> Result<Self::Response, ApiError> {
        let _conn = connection(&_req);

        let _response = UserView::get_collection(
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
