use maud::{html, Markup, DOCTYPE};

/// Function used to render the basic page stucture.
///
/// # Arguments
/// * `title`: The title of the page.
/// * `body`: The page content.
///
/// # Returns
/// The composed page markup.
pub fn Page(title: impl AsRef<str>, body: Markup) -> Markup {
    let title = format!("Open Fork - {}", title.as_ref());

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link rel="stylesheet" href="assets/dist/index.css";
            }

            body {
                main {
                    h1 { "Open Fork" }

                    (body)
                }

                script type="module" src="assets/dist/index.js" {}
            }
        }
    }
}

