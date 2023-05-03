use crate::{ Crud, ManagementAsyncTrait};
use db_schema::{
    user::{User, UserData, UserError, UserForm},
    UserId,
};
use diesel::{dsl::*, *};
use errors_lib_rs::{db::ModelErrorMessage, model::ModelError};

impl Crud<UserForm, UserId> for User {
    fn create(_conn: &PgConnection, _form: &UserForm) -> Result<Self, ModelError> {
        let _result: Result<User, result::Error> = insert_into(users)
            .values(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(UserError::diesel_error(_err)),
        }
    }

    fn read(_conn: &PgConnection, _id: &UserId) -> Result<Self, ModelError> {
        let _result = users::table.find(_id).first::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(UserError::diesel_error(_err)),
        }
    }

    fn update(
        _conn: &PgConnection,
        _id: &UserId,
        _form: &UserForm,
    ) -> Result<Self, ModelError> {
        let _result = diesel::update(users.find(_id))
            .set(_form)
            .get_result::<Self>(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(UserError::diesel_error(_err)),
        }
    }

    fn update_only(
        _conn: &PgConnection,
        _id: &UserId,
        _form: &UserForm,
        _columns: Vec<String>,
    ) -> Result<Self, ModelError> {
        let edited_tenant = _form.clone();
        Self::update(_conn, _id, &edited_tenant)
    }

    fn delete(_conn: &PgConnection, _id: &UserId) -> Result<usize, ModelError> {
        let _result = diesel::delete(users.find(_id)).execute(_conn);
        match _result {
            Ok(res) => Ok(res),
            Err(_err) => Err(UserError::diesel_error(_err)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<UserForm, UserId> for User {
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
        let mut l_return_count: Option<i64> = None;
        let mut s = String::new();
        let mut query = match (_q, _sort) {
            (Some(q), Some(sort)) => {
                s.push_str(q);
                s.push_str(" ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::User_by_filter(&s)
            }
            (Some(q), None) => Self::User_by_filter(q),
            (None, Some(sort)) => {
                s.push_str(" 1=1 ORDER BY ");
                s.push_str(&sort.join(", "));
                Self::User_by_filter(&s)
            }
            (None, None) => users.into_boxed(),
        };
        if let Some(total_count) = _total_count {
            if *total_count {
                let _total_sql = match _q {
                    Some(q) => Self::User_by_filter(q),
                    None => users.into_boxed(),
                };
                let _result = _total_sql.select(count_star()).first::<i64>(_conn);

                match _result {
                    Ok(res) => l_return_count = Some(res),
                    Err(_err) => return Err(UserError::diesel_error(_err)),
                }
            }
        }
        if let Some(offset) = _offset {
            query = query.offset(*offset);
        }
        if let Some(limit) = _limit {
            query = query.limit(*limit + 1);
        }
        let mut _result = query.load::<User>(_conn);
        let mut _data = match _result {
            Ok(res) => res,
            Err(_err) => return Err(UserError::diesel_error(_err)),
        };
        let l_has_more = _data.len() > _limit.unwrap() as usize;
        let mut _l_limit: usize;
        if l_has_more {
            _l_limit = _limit.unwrap() as usize;
            _data = (_data[.._l_limit]).to_vec();
        }
        Ok((_data, l_return_count, l_has_more))
    }
    async fn get_item(
        _conn: &PgConnection,
        _id: &UserId,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        User::read(_conn, _id)
    }
    async fn create_item(
        _conn: &PgConnection,
        _form: &UserForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        User::create(_conn, _form)
    }
    async fn update_item(
        _conn: &PgConnection,
        _id: &UserId,
        _form: &UserForm,
        _fields: &Option<Vec<String>>,
        _expand: &Option<Vec<String>>,
    ) -> Result<Self, ModelError>
    where
        Self: Sized,
    {
        User::update(_conn, _id, _form)
    }
    async fn delete_item(_conn: &PgConnection, _id: &UserId) -> Result<usize, ModelError> {
        User::delete(_conn, _id)
    }
}

/// CRUD implementation for Tenant Identity Provider
///
/// Create, Update, Delete, Read operation implementation
/// fn create(
///     _conn: &PgConnection,
///     _form: &TenantIdentityProviderForm
/// ) -> Result<Self, Error>
///

// impl Aggregation<UserId> for UserAggregation {
//     fn read(_conn: &PgConnection, _id: UserId) -> Result<Self, Error> {
//         client_aggregations
//             .filter(client_aggregations::client_id.eq(_id))
//             .first::<Self>(_conn)
//     }
// }

#[async_trait::async_trait(?Send)]
pub trait User_ {
    /// Helper functions
    fn User_by_filter<'a>(_sql: &'a str) -> Users::BoxedQuery<'a, diesel::pg::Pg> {
        Users::table.filter(sql(_sql)).into_boxed()
    }
    async fn get_items(
        _conn: &PgConnection,
        _Users_id: &Vec<UserId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &(bool, bool),
    ) -> Result<Vec<UserData>, ModelError>
    where
        Self: Sized;
}

#[async_trait::async_trait(?Send)]
impl User_ for User {
    async fn get_items(
        _conn: &PgConnection,
        _Users_id: &Vec<UserId>,
        _offset: &Option<i64>,
        _limit: &Option<i64>,
        _expand: &bool,
    ) -> Result<Vec<UserData>, ModelError>
    where
        Self: Sized,
    {
        let _result = Users::table
            .filter(users::id.eq_any(_Users_id))
            .limit(_limit.unwrap_or(10))
            .offset(_offset.unwrap_or(0))
            .load::<User>(_conn);
        match _result {
            Ok(items) => {
                let mut _user_data = vec![];
                for element in items {
                    let _item = UserData {
                        id: element.0.id,
                        fname: element.0.fname,
                        lname: element.0.lname,
                        gender: element.1.gender,
                        email: element.0.email,
                        address: element.0.address,
                        type_: element.0.type_,
                        created_by: element.0.created_by,
                        created_at: element.0.created_at,
                        updated_by: element.0.updated_by,
                        updated_at: element.0.updated_at,
                    };
                    _user_data.push(_item);
                }

                Ok(_user_data)
            }
            Err(_err) => return Err(UserError::diesel_error(_err)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl ManagementAsyncTrait<UserForm, UserId> for UserData {}
