use crate::models::database::DbType;

use super::{Applyable, TableField, TableName};

pub trait QueryBuilderExt<'args, DB>
where
    DB: sqlx::Database,
{
    fn select(&mut self, table: &TableName, fields: &[impl TableField]);
    fn push_offset<T: Into<i64>>(&mut self, offset: T);
    fn push_limit<T: Into<i64>>(&mut self, limit: T);
    fn push_and_where<'qb, F>(&'qb mut self, clause_fn: F)
    where
        F: FnOnce(&mut sqlx::query_builder::Separated<'qb, 'args, DB, &str>);
    fn apply(&mut self, applyable: &impl Applyable);
}

impl<'args> QueryBuilderExt<'args, DbType> for sqlx::QueryBuilder<'args, DbType> {
    fn select(&mut self, table: &TableName, fields: &[impl TableField]) {
        self.push("SELECT ");
        {
            let mut sep = self.separated(", ");
            for field in fields {
                sep.push(format!(r#""{}""#, field));
            }
        }
        self.push(format!(r#" FROM "{}""#, table));
    }

    fn push_offset<T: Into<i64>>(&mut self, offset: T) {
        self.push(" OFFSET ");
        self.push_bind(offset.into());
    }

    fn push_limit<T: Into<i64>>(&mut self, limit: T) {
        self.push(" LIMIT ");
        self.push_bind(limit.into());
    }

    fn push_and_where<'qb, F>(&'qb mut self, clause_fn: F)
    where
        F: FnOnce(&mut sqlx::query_builder::Separated<'qb, 'args, DbType, &str>),
    {
        if !self.sql().contains("WHERE") {
            self.push(" WHERE ");
        }

        let mut separated = self.separated(" AND ");
        clause_fn(&mut separated)
    }

    fn apply(&mut self, applyable: &impl Applyable) {
        applyable.apply(self);
    }
}
