use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::schema::recipes;

mod filter;
mod list;

pub use filter::RecipeFilterOptions;
pub use list::RecipeList;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = recipes, primary_key(id))]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
}
