use actix_web::{get, web, HttpResponse, Responder};
use mime_guess::mime;

use crate::STATIC_ASSETS;

#[get("/assets/{path:.*}")]
pub async fn get(path: web::Path<String>) -> impl Responder {
    let asset_path = path.into_inner();
    log::debug!("Serving static asset: {asset_path}");

    match STATIC_ASSETS.get_file(&asset_path) {
        Some(file) => {
            let content = file.contents_utf8().unwrap();
            HttpResponse::Ok()
                .content_type(mime_guess::from_path(&asset_path).first_or(mime::TEXT_PLAIN_UTF_8))
                .body(content)
        }
        None => HttpResponse::NotFound().finish(),
    }
}
