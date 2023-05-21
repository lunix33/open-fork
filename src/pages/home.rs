use std::sync::Arc;

use actix_web::{
    web::{Data, Query, ServiceConfig},
    Responder,
};
use maud::html;

use crate::{
    models::{
        database::DbPool,
        pagination::PaginationOptions,
        recipe::{RecipeFilterOptions, RecipeList},
    },
    layout::Page, ApplicationError, components::card::Card,
};

pub fn configure(config: &mut ServiceConfig) {
    config.service(home);
    config.service(filtered);
}

#[actix_web::get("/")]
async fn home(conn: Data<DbPool>, pagination: Option<Query<PaginationOptions>>) -> impl Responder {
    let pagination = pagination.map(|p| p.into_inner());
    let filters = RecipeFilterOptions::default();
    match futures::try_join!(
        RecipeList::filter(&conn, &filters, pagination),
        RecipeList::count(&conn, &filters)
    )
    .map_err(|e| ApplicationError::Db(Arc::new(e)))
    {
        Err(e) => Err(e),
        Ok((recipes, count)) => Ok(render(recipes, count)),
    }
}

#[actix_web::post("/")]
async fn filtered(
    conn: Data<DbPool>,
    pagination: Option<Query<PaginationOptions>>,
    //filter goes here!
) -> impl Responder {
    let pagination = pagination.map(|p| p.into_inner());
    let filters = RecipeFilterOptions::default();
    match futures::try_join!(
        RecipeList::filter(&conn, &filters, pagination),
        RecipeList::count(&conn, &filters)
    )
    .map_err(|e| ApplicationError::Db(Arc::new(e)))
    {
        Err(e) => Err(e),
        Ok((recipes, count)) => Ok(render(recipes, count)),
    }
}

pub fn render(recipes: Vec<RecipeList>, total: i64) -> impl Responder {
    Page(
        "Home",
        html! {
            section.recipes {
                @for r in &recipes {
                    (Card(html! {
                        h2 { (r.name) }
                        p {
                            (r.updated_on)
                        }
                    }, None))
                }
            }
            "Total count:"
            (total)
        },
    )
}
