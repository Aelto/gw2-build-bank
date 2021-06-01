use maud::{html, Markup, DOCTYPE};
use crate::components::header::header;
use crate::components::menu::menu;

pub fn page(page_title: &str, page_content: &Markup) -> Markup {
  html! {
    (DOCTYPE)
    html lang="en" {
      (header(page_title))

      body {
        (menu(page_title))

        div id="content" {
          (page_content)
        }

        div id="background" {
          img.pattern_1 src="/assets/pattern3.jpg";
          img.pattern_2 src="/assets/pattern5.jpg";
          img.pattern_3 src="/assets/HoT_Rytlock_KeyArt_Final.png";
        }
      }
    }
  }
}