use db_schema::models::categories::{Category, CategoryAggregation, CategoryForm};
use db_schema::{naive_now, schema::categories, CategoryId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct CategoryApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct CategoryRequest {
    pub name: Option<String>,
    pub parent: Option<String>,
}
//
//
//
