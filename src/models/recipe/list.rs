use chrono::NaiveDateTime;
use diesel::{expression::AsExpression, prelude::*};

use super::{Recipe, RecipeFilterOptions};
use crate::models::{
    database::{DbBackend, DbConnection},
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
    pub fn filter(conn: &mut DbConnection, filter: RecipeFilterOptions) -> QueryResult<Vec<Self>> {
        use self::recipes::dsl::*;

        let mut query = recipes.into_boxed::<DbBackend>();
        query = query.order_by(created_on);
        query = query.filter(filter.as_expression());

        query.load::<Self>(conn)
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
