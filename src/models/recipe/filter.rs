use chrono::NaiveDateTime;
use diesel::{expression::AsExpression, prelude::*, sql_types};

use crate::models::database::DbBackend;

use super::recipes;

pub struct RecipeFilterOptions {
    pub name: Option<String>,
    pub has_image: Option<bool>,
    pub last_updated: Option<NaiveDateTime>,
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

type FilterExpressionType = sql_types::Bool;
type FilterExpression =
    Box<dyn BoxableExpression<recipes::table, DbBackend, SqlType = FilterExpressionType>>;

impl AsExpression<FilterExpressionType> for RecipeFilterOptions {
    type Expression = FilterExpression;

    fn as_expression(self) -> Self::Expression {
        use self::recipes::dsl::*;

        let mut expr = None;
        if let Some(name_filter) = self.name {
            expr = attach_expression(expr, Box::new(name.like(format!("%{name_filter}%"))));
        }
        if let Some(true) = self.has_image {
            expr = attach_expression(expr, Box::new(image.is_not_null()));
        }
        if let Some(time) = self.last_updated {
            expr = attach_expression(expr, Box::new(updated_on.ge(time)))
        }

        match expr {
            Some(e) => e,
            None => Box::new(<bool as AsExpression<FilterExpressionType>>::as_expression(
                true,
            )),
        }
    }
}

fn attach_expression(
    current: Option<FilterExpression>,
    expr: FilterExpression,
) -> Option<FilterExpression> {
    match current {
        Some(c) => Some(Box::new(c.and(expr))),
        None => Some(Box::new(expr)),
    }
}
