use crate::{
    schema::{user_aggregations, users},
    UserAggregationId, UserId,
};

use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

// struct n schema iin deer bairlah dawhraga boloi.
//
// tuhain struct deer zuwshuurugduh uildiid.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
// schema deerh table name.
#[table_name = "users"]
// schema ruu oroh layer.
pub struct User {
    // lib deer generate code hiij awna. 
    pub id: UserId,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub email: String,
    pub phone_number: String,
    // null utga awj bolno = Option<...>.
    pub address: Option<String>,
    pub user_name: String,
    pub pwd: String,
    pub type_: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
// burtgel awah struct buyu request iin struct iin door orshino.
// UserForm struct --> User struct --> users schema --> users table on postgresql.
#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "users"]
pub struct UserForm {
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub user_name: Option<String>,
    pub pwd: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
#[derive(Clone, Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
#[belongs_to(User)]
#[table_name = "user_aggregations"]
pub struct UserAggregation {
    pub id: UserAggregationId,
    pub user_id: UserId,
    pub orders: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
#[derive(
    Clone,Debug,Serialize,Deserialize,PartialEq, Queryable,Identifiable,Insertable,Associations,
)]
#[table_name = "users"]
pub struct UserData {
    pub id: UserId,
    pub fname: String,
    pub lname: String,
    pub gender: String,
    pub email: String,
    pub address: String,
    pub type_: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
