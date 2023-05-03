use crate::{
    schema::{order_items, usr_orders},
    OrderId, OrderItemId, ProductId, UserId,
};

use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "order_items"]
pub struct OrderItem {
    pub id: OrderItemId,
    pub order_id: OrderId,
    pub product_id: ProductId,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "order_items"]
pub struct OrderItemForm {
    pub order_id: OrderId,
    pub product_id: ProductId,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
