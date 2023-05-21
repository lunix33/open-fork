use chrono::NaiveDateTime;
use sqlx::FromRow;

mod filter;
mod list;

pub use filter::RecipeFilterOptions;
pub use list::RecipeList;

use super::ext::{TableField, TableName};

pub const TABLE: TableName = TableName("recipes");
#[derive(Debug, Clone)]
pub enum Fields {
    Id,
    Name,
    Image,
    ImageMime,
    CreatedOn,
    UpdatedOn,
}
impl std::fmt::Display for Fields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Id => "id",
                Self::Name => "name",
                Self::Image => "image",
                Self::ImageMime => "image_mime",
                Self::CreatedOn => "created_on",
                Self::UpdatedOn => "updated_on",
            }
        )
    }
}
impl TableField for Fields {}

#[derive(Debug, Clone, FromRow)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub image_mime: Option<String>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
}
