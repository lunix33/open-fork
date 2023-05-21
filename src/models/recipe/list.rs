use chrono::NaiveDateTime;
use sqlx::{FromRow, Row};

use super::RecipeFilterOptions;
use crate::models::{
    database::DbPool,
    ext::{Applyable, QueryBuilderExt},
    pagination::PaginationOptions,
};

#[derive(Debug, Clone, FromRow)]
pub struct RecipeList {
    pub id: String,
    pub name: String,
    pub image: Option<Vec<u8>>,
    pub image_mime: Option<String>,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
}

impl RecipeList {
    pub async fn filter(
        conn: &DbPool,
        filter: &RecipeFilterOptions,
        pagination: Option<PaginationOptions>,
    ) -> sqlx::Result<Vec<Self>> {
        let mut query = sqlx::QueryBuilder::new("");
        query.select(
            &super::TABLE,
            &[
                super::Fields::Id,
                super::Fields::Name,
                super::Fields::Image,
                super::Fields::ImageMime,
                super::Fields::CreatedOn,
                super::Fields::UpdatedOn,
            ],
        );

        filter.apply(&mut query);
        if let Some(pagination) = pagination {
            query.apply(&pagination);
        }

        query.build_query_as::<Self>().fetch_all(conn).await
    }

    pub async fn count(conn: &DbPool, filter: &RecipeFilterOptions) -> sqlx::Result<i64> {
        let mut query =
            sqlx::QueryBuilder::new(format!(r#"SELECT COUNT(*) FROM "{}""#, super::TABLE));
        filter.apply(&mut query);

        query.build().fetch_one(conn).await?.try_get::<i64, _>(0)
    }
}
