use chrono::NaiveDateTime;
use sqlx::QueryBuilder;

use crate::models::{
    database::DbType,
    ext::{Applyable, QueryBuilderExt},
};

pub struct RecipeFilterOptions {
    pub name: Option<String>,
    pub has_image: Option<bool>,
    pub last_updated: Option<NaiveDateTime>,
}

impl RecipeFilterOptions {
    pub fn has_filers(&self) -> bool {
        return self.name.is_some() || self.has_image.is_some() || self.last_updated.is_some();
    }
}

impl Applyable for RecipeFilterOptions {
    fn apply<'a>(&self, query: &mut QueryBuilder<'a, DbType>) {
        if !self.has_filers() {
            return;
        }
        query.push_and_where(|sep| {
            if let Some(name) = &self.name {
                let name_query = format!("%{name}%");
                sep.push_unseparated(&format!(r#""{}" LIKE "#, super::Fields::Name));
                sep.push_bind(name_query);
            }

            if let Some(true) = &self.has_image {
                sep.push(&format!(r#""{}" IS NOT NULL"#, super::Fields::Image));
            }

            if let Some(last_updated) = &self.last_updated {
                sep.push_unseparated(&format!(r#""{}" >= "#, super::Fields::UpdatedOn));
                sep.push_bind(last_updated.clone());
            }
        });
    }
}

impl Default for RecipeFilterOptions {
    fn default() -> Self {
        Self {
            name: None,
            has_image: None,
            last_updated: None,
        }
    }
}
