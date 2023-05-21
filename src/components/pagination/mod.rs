use std::cmp::max;

use maud::{Markup, html};

use crate::models::pagination::PaginationOptions;

const AROUND: i64 = 4;

pub fn Pagination(pagination: &PaginationOptions, total: i64) -> Markup {
    let page = pagination.page as i64;
    let page_count = ((total as f64) / (page as f64)).ceil() as i64;
    let from = max(page - AROUND, 0i64);
    let to = min(page + AROUND, page_count)
    
    html! {
        div {
            @if pagination.page > 0 {
                button { "Previous" }
            }

            @for p in from..to {
                @let href = if p == page { "#".to_string } else { format!("...") } 
                a { (p + 1) }
            }

            @if (pagination.page as i64) < (page_count - 1) {
                button { "Next" }
            }
        }
    }
}
