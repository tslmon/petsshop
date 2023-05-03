#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_newtype;
pub mod models;
pub mod schema;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

pub type DbPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn naive_now() -> NaiveDateTime {
    chrono::prelude::Utc::now().naive_utc()
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct UserId(pub String);
impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct UserAggregationId(pub String);
impl fmt::Display for UserAggregationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CategoryId(pub String);
impl fmt::Display for CategoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CategoryAggregationId(pub String);
impl fmt::Display for CategoryAggregationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct ProductId(pub String);
impl fmt::Display for ProductId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct OrderId(pub String);
impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct OrderItemId(pub String);
impl fmt::Display for OrderItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct PaymentId(pub String);
impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CommentId(pub String);
impl fmt::Display for CommentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, DieselNewType)]
pub struct CardId(pub String);
impl fmt::Display for CardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
