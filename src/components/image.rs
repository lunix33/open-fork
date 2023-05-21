use base64::Engine;
use maud::{Markup, html};
use mime_guess::Mime;

pub enum ImageContent<'a> {
    Binary(Mime, &'a [u8]),
    Url(&'a str),
}

pub fn Image(content: ImageContent, alt: &str) -> Markup {
    use ImageContent::*;
    match content {
        Binary(mime, data) => {
            let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data);
            let url = format!("data:{},{}", mime.to_string(), b64);
            html! {
                img src=(url) alt=(alt) {}
            }
        },
        Url(url) => html! {
            img src=(url) alt=(alt) {}
        }
    }
}
