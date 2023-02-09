//! Module containing renderes used to render static asset files.
use maud::{html, PreEscaped, Render};

use crate::STATIC_ASSETS;

/// Renderer tuple to render an asset script.
///
/// # Elements
/// 1. The static asset path.
pub struct Script<'a>(pub &'a str);

impl<'a> Render for Script<'a> {
    fn render(&self) -> maud::Markup {
        let script_str = get_asset(self.0);

        html! {
            script {
                (PreEscaped(script_str))
            }
        }
    }
}

/// Renderer tuple to render an asset stylesheet.
///
/// # Elements
/// 1. The static asset path.
pub struct Style<'a>(pub &'a str);

impl<'a> Render for Style<'a> {
    fn render(&self) -> maud::Markup {
        let style_str = get_asset(self.0);

        html! {
            style {
                (PreEscaped(style_str))
            }
        }
    }
}

/// Get a static asset content by path.
#[inline]
fn get_asset<'a>(path: &str) -> &'a str {
    STATIC_ASSETS
        .get_file(path)
        .unwrap()
        .contents_utf8()
        .unwrap()
}
