
use actix_web::body::Body;
use actix_web::{web, App, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };
      HttpResponse::Ok().content_type(from_path(path).first_or_octet_stream().as_ref()).body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

pub fn asset_handler(path: web::Path<String>) -> HttpResponse {
  handle_embedded_file(&path.0)
}