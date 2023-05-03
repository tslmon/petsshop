use crate::common::*;
use actix_session::Session;
use actix_web::body::MessageBody;
use actix_web::web::Json;
use actix_web::{
    body::BoxBody,
    http::{header::ContentType, StatusCode},
    web, HttpResponse,
};
use actix_web::{
    error::ErrorBadRequest, web::Data, web::Query, Error, FromRequest, HttpRequest, Responder,
};
use errors_lib_rs::model::{ApiError, ApiErrorEnum};
use futures::{
    executor::block_on,
    future::{err, ok, Ready},
    try_join,
};
use std::collections::HashMap;
use std::str::FromStr;
use utils::{AuthError, QUERY_EXPAND_REGEX, QUERY_FIELDS_REGEX, QUERY_SORT_REGEX};

//
pub mod cards;
pub mod categories;
pub mod comments;
pub mod health;
pub mod order_items;
pub mod orders;
pub mod payments;
pub mod products;
pub mod users;
//
#[async_trait::async_trait(?Send)]
pub trait Perform {
    type Response: serde::ser::Serialize + Send;

    async fn perform(&self, context: &Data<AuthContext>) -> Result<Self::Response, AuthError>;
}

#[async_trait::async_trait(?Send)]
pub trait ManagementCreate {
    type Response: Responder;
    async fn perform(
        &self,
        _req: &HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, AuthError>;
}

#[async_trait::async_trait(?Send)]
pub trait ManagementUpdate {
    type Response: Responder;
    async fn perform(
        &self,
        _req: &HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, AuthError>;
}

#[async_trait::async_trait(?Send)]
pub trait ManagementDelete {
    type Response: Responder;
    async fn perform(
        &self,
        _req: &HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, AuthError>;
}

#[async_trait::async_trait(?Send)]
pub trait ManagementCollection {
    type Response: Responder;
    async fn perform(
        &self,
        _req: &HttpRequest,
        _coll: CollectionRequest,
    ) -> Result<Self::Response, AuthError>;
}

#[async_trait::async_trait(?Send)]
pub trait ManagementTrait<T: Sized + Send + 'static> {
    type Response: Responder;
    async fn create_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<T>,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn update_item(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<T>,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn delete_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn delete_collection(
        _req: HttpRequest,
        _single: SingularRequest,
        _data: Json<T>,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn get_item(
        _req: HttpRequest,
        _single: SingularRequest,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
    async fn get_collection(
        _req: HttpRequest,
        _coll: CollectionRequest,
    ) -> Result<Self::Response, ApiError> {
        todo!()
    }
}

#[async_trait::async_trait(?Send)]
pub trait PerformWithParams {
    type Response: serde::ser::Serialize + Send;

    async fn perform_with_params(
        &self,
        path: String,
        context: &Data<AuthContext>,
    ) -> Result<Self::Response, AuthError>;
}

impl Default for SingularRequest {
    fn default() -> Self {
        Self {
            fields: None,
            expand: None,
        }
    }
}

impl SingularRequest {
    fn set_expand_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_expand) = &value {
            if QUERY_EXPAND_REGEX.is_match(&_expand) {
                for mut item in _expand
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _expand_str = String::new();
                    if let Some(val) = item.next() {
                        _expand_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Expand parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Expand parameter ".to_owned() + &_expand_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_expand_str) {
                        _return_vec.push(_expand_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Expand parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.expand = Some(_return_vec);
        }
        Ok(self)
    }

    fn set_fields_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_fields) = &value {
            if QUERY_FIELDS_REGEX.is_match(&_fields) {
                for mut item in _fields
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _fields_str = String::new();
                    if let Some(val) = item.next() {
                        _fields_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Fields parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Fields parameter ".to_owned() + &_fields_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_fields_str) {
                        _return_vec.push(_fields_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Fields parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.fields = Some(_return_vec);
        }
        Ok(self)
    }
}

impl FromRequest for SingularRequest {
    type Error = ApiError;
    type Future = Ready<Result<Self, ApiError>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
        let mut l_return = Self::default();
        match l_return.set_fields_from_str(query.get("fields")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        match l_return.set_expand_from_str(query.get("expand")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        ok(l_return)
    }
}

impl Default for CollectionRequest {
    fn default() -> Self {
        Self {
            fields: None,
            expand: None,
            q: None,
            total_count: None,
            sort: None,
            offset: Some(0),
            limit: Some(10),
        }
    }
}

impl CollectionRequest {
    fn set_sort_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_sort) = &value {
            if QUERY_SORT_REGEX.is_match(&_sort) {
                for mut item in _sort
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _sort_str = String::new();
                    if let Some(val) = item.next() {
                        _sort_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Sort parameter contains empty field."),
                        ));
                    };
                    if let Some(val) = item.next() {
                        if "asc".eq_ignore_ascii_case(val) || "desc".eq_ignore_ascii_case(val) {
                            _sort_str.push_str(" ");
                            _sort_str.push_str(&val);
                        } else {
                            return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            format!("Sort parameter must use 'asc' for ascending or 'desc' for descending. Wrong field name: '{}'", _sort_str),
                        ));
                        }
                    } else {
                        _sort_str.push_str(" asc");
                    };
                    if !_return_vec.iter().any(|a| a == &_sort_str) {
                        _return_vec.push(_sort_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Sort parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.sort = Some(_return_vec);
        }
        Ok(self)
    }

    fn set_expand_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_expand) = &value {
            if QUERY_EXPAND_REGEX.is_match(&_expand) {
                for mut item in _expand
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _expand_str = String::new();
                    if let Some(val) = item.next() {
                        _expand_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Expand parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Expand parameter ".to_owned() + &_expand_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_expand_str) {
                        _return_vec.push(_expand_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Expand parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.expand = Some(_return_vec);
        }
        Ok(self)
    }

    fn set_fields_from_str(&mut self, value: Option<&String>) -> Result<&Self, ApiError> {
        let mut _return_vec: Vec<String> = Vec::new();
        if let Some(_fields) = &value {
            if QUERY_FIELDS_REGEX.is_match(&_fields) {
                for mut item in _fields
                    .to_lowercase()
                    .split(',')
                    .into_iter()
                    .map(|a| a.split_whitespace())
                {
                    let mut _fields_str = String::new();
                    if let Some(val) = item.next() {
                        _fields_str.push_str(&val);
                    } else {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            String::from("Fields parameter included empty value."),
                        ));
                    };
                    if let Some(_) = item.next() {
                        return Err(ApiError::new(
                            ApiErrorEnum::InvalidRequest,
                            String::from("invalid_request"),
                            "Fields parameter ".to_owned() + &_fields_str + " was wrong.",
                        ));
                    };
                    if !_return_vec.iter().any(|a| a == &_fields_str) {
                        _return_vec.push(_fields_str);
                    }
                }
            } else {
                return Err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Fields parameter's syntax was wrong. Please use (a-z, A-Z, '_', ',').",
                    ),
                ));
            }
            self.fields = Some(_return_vec);
        }
        Ok(self)
    }
}

impl FromRequest for CollectionRequest {
    type Error = ApiError;
    type Future = Ready<Result<Self, ApiError>>;
    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();
        let mut l_return = Self::default();
        if let Some(l_q) = query.get("q") {
            if !l_q.is_empty() {
                l_return.q = Some(l_q.to_string());
            } else {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from("Q parameter included empty value."),
                ));
            }
        };
        if let Some(l_total_count) = query.get("total_count") {
            l_return.total_count = Some("true".eq_ignore_ascii_case(l_total_count))
        };
        if let Some(l_offset) = query.get("offset") {
            let var = i64::from_str_radix(l_offset, 10);
            if var.is_err() {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from("Offset parameter parse error. Invalid data type."),
                ));
            }
            if var.clone().ok().unwrap() < 0 {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Offset parameter shouldn't be a negative value. Invalid data type.",
                    ),
                ));
            }
            l_return.offset = var.ok();
        };
        if let Some(l_limit) = query.get("limit") {
            let var = i64::from_str_radix(l_limit, 10);
            if var.is_err() {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from("Limit parameter parse error. Invalid data type."),
                ));
            }
            if var.clone().ok().unwrap() <= 0 {
                return err(ApiError::new(
                    ApiErrorEnum::InvalidRequest,
                    String::from("invalid_request"),
                    String::from(
                        "Limit parameter shouldn't be a negative value. Invalid data type.",
                    ),
                ));
            }
            l_return.limit = var.ok();
        };
        match l_return.set_sort_from_str(query.get("sort")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        match l_return.set_expand_from_str(query.get("expand")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        match l_return.set_fields_from_str(query.get("fields")) {
            Ok(_) => (),
            Err(e) => return err(e),
        };
        ok(l_return)
    }
}

// #[async_trait::async_trait(?Send)]
// pub trait PerformWithQuery {
//     type Response: serde::ser::Serialize + Send;

//     async fn perform_with_query(
//         &self,
//         query: web::Query<Token>,
//         context: &Data<AuthContext>,
//     ) -> Result<Self::Response, AuthError>;
// }

#[async_trait::async_trait(?Send)]
pub trait PerformWithSession {
    type Response: serde::ser::Serialize + Send;

    async fn perform_with_session(
        &self,
        context: &Data<AuthContext>,
        session: Session,
    ) -> Result<Self::Response, AuthError>;
}

// fn response_result(_result: Result<String, Error>, _status: u16, _reason: String) -> HttpResponseBuilder {
//   let mut _return = HttpResponseBuilder::new();

//   _return = _return.status(StatusCode::from_u16(200)).reason
//   match _result {
//     Ok(_) => {
//       let _body = _result.unwrap();
//       println!("_body={:?}", _body);
//       HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .body(_body)
//     }
//     Err(_err) => {
//       // HttpResponse::from(err),
//       let _body = BoxBody::new(_err.to_string());
//       let _status = StatusCode::from_u16(500).unwrap();
//       HttpResponse::with_body(_status, _body)
//     }
//   }
// }

//
// OpenID Connect Discovery. this is test
//

#[async_trait::async_trait(?Send)]
pub trait OpenIDConnectDiscoverySingular {
    type Response: Responder;
    async fn perform(&self, _req: &HttpRequest) -> Result<Self::Response, AuthError>;
}

#[async_trait::async_trait(?Send)]
pub trait JSONWebKeySetSingular {
    type Response: Responder;
    async fn perform(&self, _req: &HttpRequest) -> Result<Self::Response, AuthError>;
}

#[cfg(test)]
mod tests {
    // use crate::actions::CollectionRequest;
    // use actix_web::Error;
    #[test]
    fn it_works() {}

    // // #[test]
    // #[tokio::test]
    // async fn test_sort() -> Result<(), Error> {
    //     let mut _coll = CollectionRequest::default();
    //     _coll
    //         .set_sort_from_str(Some(&String::from(
    //             "id,     name   desc, asdf asc   , id asc",
    //         )))
    //         .await?;
    //     //assert_eq!("id asc, name desc, asdf asc", _coll.sort.unwrap().join(", "));
    //     assert_eq!(vec!["id asc", "name desc", "asdf asc"], _coll.sort.unwrap());
    //     // _coll.sort = Some(String::from("id,   "));
    //     // let vec = _coll.get_sort_vector().await?;
    //     // assert_eq!(vec!["id"], vec);
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_fields() -> Result<(), Error> {
    //     let mut _coll = CollectionRequest::default();
    //     _coll
    //         .set_fields_from_str(Some(&String::from("id,     name.desc,asdfasc")))
    //         .await?;
    //     assert_eq!(vec!["id", "name.desc", "asdfasc"], _coll.fields.unwrap());
    //     // _coll.sort = Some(String::from("id,   "));
    //     // let vec = _coll.get_sort_vector().await?;
    //     // assert_eq!(vec!["id"], vec);
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_expand() -> Result<(), Error> {
    //     let mut _coll = CollectionRequest::default();
    //     _coll
    //         .set_expand_from_str(Some(&String::from(
    //             "id, asdfasdf,asdfasdf,asdfasdf,asdfasdf ,   name.desc,asdfasc,asdfasdf,asdfasdf",
    //         )))
    //         .await?;
    //     assert_eq!(
    //         vec!["id", "asdfasdf", "name.desc", "asdfasc"],
    //         _coll.expand.unwrap()
    //     );
    //     // _coll.sort = Some(String::from("id,   "));
    //     // let vec = _coll.get_sort_vector().await?;
    //     // assert_eq!(vec!["id"], vec);
    //     Ok(())
    // }
}
