use db_schema::models::cards::{Card, CardForm};
use db_schema::{naive_now, schema::cards, CardId, CategoryId, ProductId, UserId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct CardApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct CardRequest {
    pub user_id: UserId,
    pub product_id: ProductId,
}
//
//
//
