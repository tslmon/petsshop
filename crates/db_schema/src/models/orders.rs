use crate::{
    schema::{order_items, usr_orders},
    OrderId, OrderItemId, ProductId, UserId,
};
use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "usr_orders"]
pub struct Order {
    pub id: OrderId,
    pub user_id: UserId,
    pub order_date: chrono::NaiveDateTime,
    pub status: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "usr_orders"]
pub struct OrderForm {
    pub user_id: Option<UserId>,
    pub status: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
