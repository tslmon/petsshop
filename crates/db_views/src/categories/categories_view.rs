use crate::ResponceCollection;
use db_queries::{models::categories::categories::Category_, ManagementAsyncTrait, ViewToVec};
use db_schema::{
    models::categories::{Category, CategoryAggregation, CategoryForm},
    CategoryId,
};
use diesel::PgConnection;
use errors_lib_rs::model::ModelError;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Clone)]
pub struct CategoryView {
    #[serde(flatten)]
    pub item: Value,
}

type CategoryViewTuple = (Category);

impl ViewToVec for CategoryView {
    type DbTuple = CategoryViewTuple;
    fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
        items
            .iter()
            .map(|a| Self {
                item: json!(a.to_owned()),
            })
            .collect::<Vec<Self>>()
    }
}

impl CategoryView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &CategoryForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<CategoryView, ModelError> {
        let _item = Category::create_item(_conn, _form, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;
        let vec = vec![_item];

        Ok(Self { item: _return_item })
    }

    pub async fn update_item(
        _conn: &PgConnection,
        _id: &CategoryId,
        _form: &CategoryForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let item = Category::update_item(_conn, _id, _form, &_fields, _expand).await?;

        let _return_item = item.collect_fields(&_fields).await?;
        let vec = vec![item];
        Ok(Self { item: _return_item })
    }

    pub async fn delete_item(_conn: &PgConnection, _id: &CategoryId) -> Result<usize, ModelError> {
        Category::delete_item(_conn, _id).await
    }

    pub async fn get_item(
        _conn: &PgConnection,
        _id: &CategoryId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let _item = Category::get_item(_conn, _id, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        let vec = vec![_item];

        Ok(Self { item: _return_item })
    }

    pub async fn get_collection(
        _conn: &PgConnection,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
        _q: &Option<String>,
        _sort: &Option<Vec<String>>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _total_count: &Option<bool>,
    ) -> Result<ResponceCollection<Self>, ModelError> {
        let mut _return_count: Option<i64>;

        let (items, _return_count, _has_more) = Category::get_collection(
            _conn,
            _fields,
            _expand,
            _q,
            _sort,
            _offset,
            _limit,
            _total_count,
        )
        .await?;

        let mut _return_list: Vec<CategoryView> = Vec::new();
        let mut i: usize = 0;
        for item in items.into_iter() {
            let _return_user = item.collect_fields(_fields).await?;
            _return_list.push(CategoryView { item: _return_user });
            i = i + 1;
        }

        let mut _res = ResponceCollection::<CategoryView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };

        Ok(_res)
    }
}
