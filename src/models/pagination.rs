use serde::Deserialize;
use sqlx::QueryBuilder;

use super::{
    database::DbType,
    ext::{Applyable, QueryBuilderExt},
};

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationOptions {
    pub page: u32,
    pub limit: u32,
}

impl Default for PaginationOptions {
    fn default() -> Self {
        Self { page: 0, limit: 10 }
    }
}

impl PaginationOptions {
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}

impl Applyable for PaginationOptions {
    fn apply<'a>(&self, query: &mut QueryBuilder<'a, DbType>) {
        query.push_limit(self.limit);
        query.push_offset(self.page * self.limit);
    }
}
