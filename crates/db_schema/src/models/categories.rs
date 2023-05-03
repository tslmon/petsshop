use crate::{
    schema::{categories, category_aggregations},
    CategoryAggregationId, CategoryId,
};

use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub parent: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "categories"]
pub struct CategoryForm {
    pub name: Option<String>,
    pub parent: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Clone, Queryable, Identifiable, Associations, PartialEq, Debug, Serialize)]
#[belongs_to(Category)]
#[table_name = "category_aggregations"]
pub struct CategoryAggregation {
    pub id: CategoryAggregationId,
    pub category_id: CategoryId,
    pub products: i64,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}
