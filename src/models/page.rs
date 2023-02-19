use diesel::{
    backend::Backend, internal::table_macro::BoxedSelectStatement, prelude::*,
    query_builder::AsQuery,
};

#[derive(Debug, Clone)]
pub struct PageOptions {
    before: Option<String>,
    after: Option<String>,
    limit: i64,
}

impl Default for PageOptions {
    fn default() -> Self {
        Self {
            before: None,
            after: None,
            limit: 10,
        }
    }
}
