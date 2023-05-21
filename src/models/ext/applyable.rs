use sqlx::QueryBuilder;

use crate::models::database::DbType;

pub trait Applyable {
    fn apply<'a>(&self, query: &mut QueryBuilder<'a, DbType>);
}
