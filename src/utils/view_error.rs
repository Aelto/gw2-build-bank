use crate::components;
use crate::models::build::Build;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub fn view_error(message: &str) -> HttpResponse {
  let content = html! {

    h1 { "Error" }
    p { (message) }

    style type="text/css" { (get_stylesheet()) }
  };

  let view = components::page("new build", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}

fn get_stylesheet() -> String {
  "
    
  ".to_owned()
}