use maud::{html, Markup, DOCTYPE};

pub fn page(title: impl AsRef<str>, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title.as_ref()) }
            }

            body {
                main {
                    (body)
                }
            }
        }
    }
}
