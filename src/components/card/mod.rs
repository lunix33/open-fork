use maud::{Markup, html};

use crate::components::image::Image;

use super::image::ImageContent;

pub fn Card(content: Markup, image: Option<(ImageContent, &str)>) -> Markup {
    html! {
        article.card {
            @if let Some(image) = image {
                section {
                    (Image(image.0, image.1))
                }
            }
            section {
                (content)
            }
        }
    }
}
