use db_schema::models::order_items::{OrderItem, OrderItemForm};
use db_schema::{naive_now, schema::order_items, CategoryId, OrderId, OrderItemId, ProductId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct OrderItemApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct OrderItemRequest {
    pub order_id: OrderId,
    pub product_id: ProductId,
}
//
//
//
