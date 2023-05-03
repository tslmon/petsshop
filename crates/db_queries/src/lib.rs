#[macro_use]
extern crate diesel_migrations;
extern crate diesel;
extern crate lazy_static;
extern crate strum_macros;
use diesel::PgConnection;
pub mod models;
use crate::diesel::Connection;
use db_schema::models::errors::PetsShopAPIError;
use diesel::{result::Error, *};
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::env::{self, VarError};

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
pub fn get_database_url_from_env() -> Result<String, VarError> {
    env::var("TREE_DATABASE_URL")
}

embed_migrations!();
pub fn establish_unpooled_connection() -> PgConnection {
    let db_url = match get_database_url_from_env() {
        Ok(url) => url,
        Err(e) => panic!(
            "Failed to read database URL from env var TREE_DATABASE_URL: {}",
            e
        ),
    };
    let conn = PgConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    embedded_migrations::run(&conn).expect("load migrations");
    conn
}

pub fn option_value<T>(_value: Option<T>, _expand: bool) -> Option<T> {
    let mut _result = None;
    if _expand {
        _result = _value;
    } else {
        _result = None;
    }
    _result
}

#[async_trait::async_trait(?Send)]
pub trait HelperModelTrait {
    //
    // Output : (Option<Vec<Vec<Child>>>,Option<Vec<Vec<Child>>> ..)
    //
    type Output;
    async fn expend_childs(
        _self: Vec<Self>,
        _expand: &Option<String>,
    ) -> Result<Self::Output, Error>
    where
        Self: Sized;
    async fn map_to_fields(_self: Vec<Self>, _fields: &Option<Vec<&str>>) -> Result<Value, Error>
    where
        Self: Sized;
    async fn get_child_fields<'a>(
        _child_name: str,
        _fields: &'a Option<Vec<&'a str>>,
    ) -> Result<&'a Option<Vec<&'a str>>, Error>
    where
        Self: Sized;
}
pub trait Crud<Form, CodeType> {
    fn create(_conn: &PgConnection, _form: &Form) -> Result<Self, ModelError>
    where
        Self: Sized;
    fn read(_conn: &PgConnection, _code: &CodeType) -> Result<Self, ModelError>
    where
        Self: Sized;
    fn update(_conn: &PgConnection, _code: &CodeType, _form: &Form) -> Result<Self, ModelError>
    where
        Self: Sized;
    fn update_only(
        _conn: &PgConnection,
        _code: &CodeType,
        _form: &Form,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized;
    fn delete(_conn: &PgConnection, _code: &CodeType) -> Result<usize, ModelError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

pub trait Aggregation<CodeType> {
    fn read(_conn: &PgConnection, _code: CodeType) -> Result<Self, ModelError>
    where
        Self: Sized;
}

#[async_trait::async_trait(?Send)]
pub trait ManagementAsyncTrait<Form, CodeType>
where
    Self: Serialize,
{
    async fn get_collection(
        _conn: &PgConnection,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
        _q: &Option<String>,
        _sort: &Option<Vec<String>>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _total_count: &Option<bool>,
    ) -> Result<(Vec<Self>, Option<i64>, bool), ModelError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    async fn get_item(
        _conn: &PgConnection,
        _code: &CodeType,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    async fn create_item(
        _conn: &PgConnection,
        _form: &Form,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    async fn update_item(
        _conn: &PgConnection,
        _code: &CodeType,
        _form: &Form,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    async fn delete_item(_conn: &PgConnection, _code: &CodeType) -> Result<usize, ModelError> {
        unimplemented!()
    }
    async fn collect_fields(&self, _fields: &Option<Vec<String>>) -> Result<Value, ModelError> {
        if let Some(val) = _fields {
            let mut _return = Map::new();
            let _local_val = json!(&self);
            for item in val.into_iter() {
                if !item.is_empty() {
                    let json_val = _local_val.get(&item).ok_or(Error::DatabaseError(
                        result::DatabaseErrorKind::SerializationFailure,
                        Box::new(format!("column {} does not exist.", &item))
                            as Box<dyn result::DatabaseErrorInformation + Send + Sync>,
                    ));
                    let mut _value: &Value;
                    match json_val {
                        Ok(_res) => _value = _res,
                        Err(_err) => return Err(PetsShopAPIError::diesel_error(_err)),
                    };
                    _return.insert(item.into(), json!(_value));
                }
            }
            Ok(json!(_return))
        } else {
            Ok(json!(&self))
        }
    }
}
pub trait ViewToVec {
    type DbTuple;
    fn from_tuple_to_vec(tuple: Vec<Self::DbTuple>) -> Vec<Self>
    where
        Self: Sized;
}
