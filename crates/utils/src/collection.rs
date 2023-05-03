use actix_web::{HttpRequest};
use diesel::{
    pg::Pg,
    query_builder::{AstPass, Query, QueryFragment},
    query_dsl::LoadQuery,
    sql_types, PgConnection, QueryId, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ResponceCollection<T> {
    pub count: Option<i64>,
    pub total_counts: Option<i64>,
    pub has_more: bool,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub items: Vec<T>,
}

pub struct RequestCollection<'a> {
    pub conn: &'a PgConnection,
    pub fields: Option<String>,
    pub expand: Option<String>,
    pub q: Option<String>,
    pub total_count: Option<bool>,
    pub sort: Option<String>,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl<'a> RequestCollection<'a> {
    fn request_collection_new(conn: &'a PgConnection) -> Self {
        Self {
            conn: conn,
            fields: None,
            expand: None,
            q: None,
            total_count: None,
            sort: None,
            offset: Some(0),
            limit: Some(25),
        }
    }

    pub fn request_collection(conn: &'a PgConnection, req: &HttpRequest) -> Self {
        use actix_web::web::Query;

        let query = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

        let mut l_return = Self::request_collection_new(&conn);

        if let Some(l_fields) = query.get("fields") {
            l_return.fields = Some(l_fields.to_string());
        };

        if let Some(l_expand) = query.get("expand") {
            l_return.expand = Some(l_expand.to_string());
        };

        if let Some(l_q) = query.get("q") {
            l_return.q = Some(l_q.to_string());
        };

        if query.get("total_count").is_some() {
            l_return.total_count = Some(true);
        };

        if let Some(l_sort) = query.get("sort") {
            l_return.sort = Some(l_sort.to_string());
        };

        if let Some(l_offset) = query.get("offset") {
            let var = i64::from_str_radix(l_offset, 10);
            // if var.is_err() {
            //     return err(ErrorBadRequest(
            //         "Offset parameter parse error. Invalid data type.",
            //     ));
            // }
            l_return.offset = var.ok();
        };

        if let Some(l_limit) = query.get("limit") {
            let var = i64::from_str_radix(l_limit, 10);
            // if var.is_err() {
            //     return err(ErrorBadRequest(
            //         "Limit parameter parse error. Invalid data type.",
            //     ));
            // }
            l_return.limit = var.ok();
        };

        l_return
    }

    pub fn script(&self, from: String) -> String {
        let mut script = String::new();

        if self.total_count.is_some() {
            script.push_str("SELECT *, COUNT(*) OVER() FROM ");
            script.push_str(&from);
        } else {
            script.push_str("SELECT * FROM ");
            script.push_str(&from);
        }

        if self.q.is_some() {
            script.push_str(" WHERE ");
            script.push_str(self.q.as_ref().unwrap());
        }
        if self.sort.is_some() {
            script.push_str(" ORDER BY ");
            script.push_str(self.sort.as_ref().unwrap());
        }
        if self.offset.is_some() {
            script.push_str(" OFFSET ");
            script.push_str(self.offset.unwrap().to_string().as_ref());
        }
        if self.limit.is_some() {
            script.push_str(" LIMIT ");
            script.push_str((self.limit.unwrap() + 1).to_string().as_ref());
        }
        script
    }
}

pub trait ViewTrait: Sized {
    fn view(self, script: String) -> View<Self>;
}

impl<T> ViewTrait for T {
    fn view(self, script: String) -> View<Self> {
        View {
            _table: self,
            _script: script,
        }
    }
}

#[derive(Debug, Clone, QueryId, Queryable)]
pub struct View<T> {
    _table: T,
    _script: String,
}

impl<T> View<T> {
    pub fn view_load<U>(self, conn: &PgConnection) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<PgConnection, U>,
    {
        let results = self.load::<U>(conn)?;
        Ok(results)
    }
}

impl<T: Query> Query for View<T> {
    type SqlType = T::SqlType;
}

impl<T> RunQueryDsl<PgConnection> for View<T> {}

impl<T> QueryFragment<Pg> for View<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.unsafe_to_cache_prepared();
        out.push_sql(&self._script);

        Ok(())
    }
}

pub trait ViewAndTCTrait: Sized {
    fn view_total_count(self, script: String) -> ViewTotalCount<Self>;
}

impl<T> ViewAndTCTrait for T {
    fn view_total_count(self, script: String) -> ViewTotalCount<Self> {
        ViewTotalCount {
            _table: self,
            _script: script,
        }
    }
}

#[derive(Debug, Clone, QueryId, Queryable)]
pub struct ViewTotalCount<T> {
    _table: T,
    _script: String,
}

impl<T> ViewTotalCount<T> {
    pub fn view_total_count_load<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, Option<i64>)>
    where
        Self: LoadQuery<PgConnection, (U, Option<i64>)>,
    {
        let results = self.load::<(U, Option<i64>)>(conn)?;
        let total_count = results.get(0).map(|x| x.1).unwrap_or(None);
        let records = results.into_iter().map(|x| x.0).collect();

        Ok((records, total_count))
    }
}

impl<T: Query> Query for ViewTotalCount<T> {
    type SqlType = (T::SqlType, sql_types::Nullable<sql_types::BigInt>);
}

impl<T> RunQueryDsl<PgConnection> for ViewTotalCount<T> {}

impl<T> QueryFragment<Pg> for ViewTotalCount<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.unsafe_to_cache_prepared();
        out.push_sql(&self._script);
        Ok(())
    }
}
