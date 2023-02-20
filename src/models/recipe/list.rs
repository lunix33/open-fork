use chrono::NaiveDateTime;
use diesel::{expression::AsExpression, prelude::*};

use super::{Recipe, RecipeFilterOptions};
use crate::models::{
    database::{DbBackend, DbConnection},
    pagination::PaginationOptions,
    schema::recipes,
};

#[derive(Debug, Clone, Queryable, Identifiable)]
#[diesel(table_name = recipes, primary_key(id))]
pub struct RecipeList {
    pub id: String,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
}

impl RecipeList {
    pub fn filter(
        conn: &mut DbConnection,
        filter: RecipeFilterOptions,
        pagination: Option<PaginationOptions>,
    ) -> QueryResult<Vec<Self>> {
        let mut query = recipes::table.into_boxed::<DbBackend>();
        query = query.filter(filter.as_expression());

        if let Some(pagination) = pagination {
            query = query
                .offset((pagination.page * pagination.limit).into())
                .limit(pagination.limit.into());
        }

        query.load::<Self>(conn)
    }

    pub fn count(conn: &mut DbConnection) -> QueryResult<i64> {
        recipes::table.count().get_result(conn)
    }
}

impl From<Recipe> for RecipeList {
    fn from(value: Recipe) -> Self {
        Self {
            id: value.id,
            name: value.name,
            image: value.image,
            created_on: value.created_on,
            updated_on: value.updated_on,
        }
    }
}
