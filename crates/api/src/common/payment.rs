use db_schema::models::payments::{Payment, PaymentForm};
use db_schema::{naive_now, schema::payments, CategoryId, OrderId, PaymentId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct PaymentApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct PaymentRequest {
    pub order_id: OrderId,
    pub amount: String,
}
//
//
//
