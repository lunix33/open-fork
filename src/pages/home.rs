use actix_web::{get, Responder};
use maud::html;

use crate::templates;

#[get("/")]
pub async fn render() -> impl Responder {
    templates::layout::page(
        "Hello World",
        html! {
            h1 { "It works" }
        },
    )
}
