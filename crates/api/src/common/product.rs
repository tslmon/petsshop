use db_schema::models::products::{Product, ProductForm};
use db_schema::{naive_now, schema::products, CategoryId, ProductId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct ProductApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct ProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub price: Option<i64>,
    pub quantity: Option<i64>,
    pub category_id: Option<CategoryId>,
}
//
//
//
