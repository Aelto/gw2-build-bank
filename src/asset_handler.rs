
use actix_web::body::Body;
use actix_web::{web, HttpResponse};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use wry::application::window::Icon;

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

pub fn get_icon() -> Icon {
  let (icon_rgba, icon_width, icon_height) = {
    match Asset::get("icon.ico") {
      Some(content) => {
        let buffer = match content {
          Cow::Borrowed(bytes) => Vec::from(bytes),
          Cow::Owned(bytes) => bytes,
        };

        let image = image::load_from_memory(&buffer)
        .expect("Failed to open icon path")
        .into_rgba8();

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
      },
      None => panic!("could not get the window icon")
    }
  };
  Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}