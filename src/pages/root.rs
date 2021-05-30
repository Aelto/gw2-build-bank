use crate::components;
use crate::models::build::Build;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let builds = match Build::all() {
    Err(_) => Vec::new(),
    Ok(v) => v
  };

  let content = html! {    
    (components::build_list(builds))

    style type="text/css" { (get_stylesheet()) }
  };

  let view = components::page("home", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}

fn get_stylesheet() -> String {
  "
    
  ".to_owned()
}