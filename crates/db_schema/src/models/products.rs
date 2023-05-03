use crate::{schema::products, CategoryId, ProductId};

use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "products"]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub price: i64,
    pub quantity: i64,
    pub category_id: CategoryId,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "products"]
pub struct ProductForm {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub price: Option<i64>,
    pub quantity: Option<i64>,
    pub category_id: Option<CategoryId>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
