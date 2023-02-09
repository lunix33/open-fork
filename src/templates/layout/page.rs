use maud::{html, Markup, DOCTYPE};

use crate::templates::layout::assets;

/// Function used to render the basic page stucture.
///
/// # Arguments
/// * `title`: The title of the page.
/// * `body`: The page content.
///
/// # Returns
/// The composed page markup.
pub fn page(title: impl AsRef<str>, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title.as_ref()) }
                (assets::Style("style.css"))
            }

            body {
                main {
                    (body)
                }

                (assets::Script("js/index.js"))
            }
        }
    }
}
