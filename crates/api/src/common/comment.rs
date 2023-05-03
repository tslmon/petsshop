use db_schema::models::comments::{Comment, CommentForm};
use db_schema::{naive_now, schema::comments, CategoryId, CommentId, ProductId, UserId};

// use db_views::resource_view;
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable};
use futures::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
//
//
//
#[derive(Deserialize)]
pub struct CommentApi {}
//
//
//
#[derive(Clone, Default, Deserialize, Debug, Serialize)]
pub struct CommentRequest {
    pub user_id: Option<UserId>,
    pub product_id: Option<ProductId>,
    pub comment: Option<String>,
}
//
//
//
