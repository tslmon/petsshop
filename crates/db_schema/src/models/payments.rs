use crate::{schema::payments, OrderId, PaymentId, UserId};

use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "payments"]
pub struct Payment {
    pub id: PaymentId,
    pub order_id: OrderId,
    pub amount: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "payments"]
pub struct PaymentForm {
    pub order_id: OrderId,
    pub amount: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
