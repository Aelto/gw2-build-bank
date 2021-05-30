use crate::components;
use crate::models::build::Build;
use crate::utils::view_error::view_error;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let builds = Build::all();

  if builds.is_err() {
    return view_error(&format!("could not get the builds"));
  }

  let builds = builds.unwrap();

  let content = html! {
    form method="post" action="/api/build/remove" {
      select name="build_name" {
        @for build in builds {
          option value={(build.name)} { (build.name) }
        }
      }

      input type="submit" value="remove";
    }

    style type="text/css" { (get_stylesheet()) }
  };

  let view = components::page("remove build", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}

fn get_stylesheet() -> String {
  "
  label, input[type='submit'] {
    display: block;
  }

  #content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  ".to_owned()
}