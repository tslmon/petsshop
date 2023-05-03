use crate::{get_fields, get_keys, ResponceCollection, SetExpand};
use db_queries::{users::User_, ManagementAsyncTrait, ViewToVec};
use db_schema::{
    users::{User, UserData, UserForm},
    UserId,
};
use diesel::PgConnection;
use errors_lib_rs::model::ModelError;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Serialize, Clone)]
pub struct UserView {
    #[serde(flatten)]
    pub item: Value,
}

type UserViewTuple = User;

impl ViewToVec for UserViewTuple {
    type DbTuple = UserViewTuple;
    fn from_tuple_to_vec(items: Vec<Self::DbTuple>) -> Vec<Self> {
        items
            .iter()
            .map(|a| Self {
                item: json!(a.to_owned()),
            })
            .collect::<Vec<Self>>()
    }
}

impl UserView {
    pub async fn create_item(
        _conn: &PgConnection,
        _form: &UserForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<UserView, ModelError> {
        let _item = User::create_item(_conn, _form, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        Ok(Self { item: _return_item })
    }

    pub async fn update_item(
        _conn: &PgConnection,
        _id: &UserId,
        _form: &UserForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError> {
        let item = User::update_item(_conn, _id, _form, &_fields, _expand).await?;

        let _return_item = item.collect_fields(&_fields).await?;

        Ok(Self { item: _return_item })
    }

    pub async fn delete_item(
        _conn: &PgConnection,
        _id: &UserId,
    ) -> Result<usize, ModelError> {
        User::delete_item(_conn, _id).await
    }

    pub async fn get_item(
        _conn: &PgConnection,
        _id: &UserId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
        _type: i32,
    ) -> Result<Self, ModelError> {
        let _item = User::get_item(_conn, _id, &_fields, _expand).await?;

        let _return_item = _item.collect_fields(&_fields).await?;

        let _default_data = UserData::default();

        let _get_expands = Self::get_expands(_expand, _type).await?;

        let vec_id = vec![_item.id];

        let _item = User::get_items(_conn, &vec_id, &None, &None, &_get_expands.0).await?;

        let json_value: Value = serde_json::to_value(_default_data).unwrap();

        let _data_fields = get_keys(json_value);

        let keys = if _fields.is_some() {
            _fields.to_owned().unwrap()
        } else {
            _data_fields.clone()
        };

        let _expand_vec = vec![_get_expands.0 .0, _get_expands.0 .1];

        let _get_fields = get_fields(keys, _data_fields, _expand_vec, _get_expands.1).await?;

        let _return_item = _item[0].clone().collect_fields(&_get_fields).await?;

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
        _type: i32,
    ) -> Result<ResponceCollection<Self>, ModelError> {
        let mut _return_count: Option<i64>;

        let (items, _return_count, _has_more) = User::get_collection(
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

        let mut _Users_id = vec![];

        for element in items.clone() {
            _users_id.push(element.id)
        }

        let _default_data = UserData::default();

        let _get_expands = Self::get_expands(_expand, _type).await?;

        let _item =
            User::get_items(_conn, &_users_id, &None, &None, &_get_expands.0).await?;

        let json_value: Value = serde_json::to_value(_default_data).unwrap();

        let _data_fields = get_keys(json_value);

        let keys = if _fields.is_some() {
            _fields.to_owned().unwrap()
        } else {
            _data_fields.clone()
        };

        let _expand_vec = vec![_get_expands.0 .0, _get_expands.0 .1];

        let _get_fields = get_fields(keys, _data_fields, _expand_vec, _get_expands.1).await?;

        let mut _return_list: Vec<UserView> = Vec::new();

        let mut i: usize = 0;

        for item in _item.into_iter() {
            let _return_client = item.collect_fields(&_get_fields).await?;
            _return_list.push(UserView {
                item: _return_client,
            });
            i = i + 1;
        }

        let mut _res = ResponceCollection::<UserView> {
            count: Some(_return_list.len() as i64),
            total_counts: _return_count,
            has_more: _has_more,
            offset: *_offset,
            limit: *_limit,
            items: _return_list,
        };

        Ok(_res)
    }
    async fn get_expands(
        _expand: &Option<Vec<String>>,
    ) -> Result<(bool, Vec<SetExpand>), ModelError> {
        let mut _expands = false;
        if let Some(l_expand) = _expand.clone() {
            _expands = if l_expand.iter().any(|a| a == "all") {
                true
            } else {
                l_expand.iter().any(|a| a == "owner_data")
            };
        }

        let mut _expand_arr: Vec<SetExpand> = Vec::new();

        _expand_arr.push(SetExpand(4, String::from("owner_data")));

        Ok((_expands, _expand_arr))
    }
}
