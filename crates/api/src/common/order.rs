use db_schema::models::orders::{Order, OrderForm};
use db_schema::{naive_now, schema::usr_orders, CategoryId, OrderId, UserId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct OrderApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct OrderRequest {
    pub user_id: UserId,
    pub status: String
}
