use crate::components;
use clipboard::ClipboardProvider;

use maud::html;
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub async fn render(_req: HttpRequest) -> HttpResponse {

  let template_default_value = {
    let context = clipboard::ClipboardContext::new();

    context
    .and_then(|mut context| context.get_contents())
    .and_then(|content| if content.starts_with('[') && content.ends_with(']') {
      Ok(content)
    } else {
      Ok(String::new())
    })
    .unwrap_or_else(|_| String::new())
  };

  let content = html! {

    form method="post" action="/api/build/create" {
      label for="name" { "Name" }
      input id="name" name="build_name" autocomplete="false";

      label for="tooltip" { "Tooltip" };
      textarea id="tooltip" name="build_description" autocomplete="false" {}

      label for="template" { "Template" }
      input id="template" name="build_template" autocomplete="false" value=(template_default_value);

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
    
  ".to_owned()
}