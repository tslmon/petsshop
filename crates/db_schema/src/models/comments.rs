use crate::{schema::comments, CommentId, ProductId, UserId};

use errors_lib_rs::db::ModelErrorMessage;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable, Insertable)]
#[table_name = "comments"]
pub struct Comment {
    pub id: CommentId,
    pub user_id: UserId,
    pub product_id: ProductId,
    pub comment: Option<String>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: String,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Clone, Default, Debug)]
#[table_name = "comments"]
pub struct CommentForm {
    pub user_id: Option<UserId>,
    pub product_id: Option<ProductId>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}
