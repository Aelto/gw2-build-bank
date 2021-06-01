use crate::components;
use crate::models::build::Build;
use crate::utils::view_error::view_error;

use maud::{html, PreEscaped};
use actix_web::web::HttpRequest;
use actix_web::{HttpResponse};

pub async fn render(_req: HttpRequest) -> HttpResponse {

  let builds = Build::all();

  if builds.is_err() {
    return view_error(&format!("could not get the builds"));
  }

  let mut builds = builds.unwrap();

  // because the edit puts the edit build at the end of the list.
  // This allows us to have the last edited build as the first result after a
  // reload
  builds.reverse();

  let content = html! {
    form method="post" action="/api/build/update" {
      select name="build_name" id="name" {
        @for build in builds {
          option value={(build.name)} { "[" (build.profession) "] - " (build.name) }
        }
      }

      label for="tooltip" { "Tooltip" };
      textarea id="tooltip" name="build_description" {}

      label for="template" { "Template" }
      input id="template" name="build_template";

      select id="profession" name="build_profession" {
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

      input type="submit" value="update";
    }

    script { (maud::PreEscaped(get_javascript())) }
  };

  let view = components::page("edit build", &content);
  
  HttpResponse::Ok()
  .content_type("text/html")
  .body(view.into_string())
}

fn get_javascript() -> String {
  "
    const select = document.querySelector('#name');
    const tooltip = document.querySelector('#tooltip');
    const template = document.querySelector('#template');
    const profession = document.querySelector('#profession');

    select.addEventListener('change', async e => {
      fetch_and_fill_build_data(e.target.value);
    });

    async function fetch_and_fill_build_data(build_name) {
      const response = await fetch(`/api/build/${build_name}`);
      const result = await response.json();

      console.log(result);
      
      tooltip.value = result.description;
      template.value = result.template;
      profession.value = result.profession;
    }

    fetch_and_fill_build_data(select.value);
  ".to_string()
}