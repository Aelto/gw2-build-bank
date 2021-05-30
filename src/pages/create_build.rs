use crate::components;
use crate::models::build::Build;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub async fn render(_req: HttpRequest) -> HttpResponse {
  let content = html! {

    form method="post" action="/api/build/create" {
      label for="name" { "Name" }
      input id="name" name="build_name";

      label for="template" { "Template" }
      input id="template" name="build_template";

      label for="profession" { "Profession" }
      select name="build_profession" {
        option value="mesmer" { "mesmer" }
        option value="elementalist" { "elementalist" }
        option value="necromancer" { "necromancer" }
        option value="ranger" { "ranger" }
        option value="thief" { "thief" }
        option value="engineer" { "engineer" }
        option value="warrior" { "warrior" }
        option value="guardian" { "guardian" }
        option value="revenant" { "revenant" }
      }

      input type="submit" value="create";
    }

    style type="text/css" { (get_stylesheet()) }
  };

  let view = components::page("add build", &content);
  
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