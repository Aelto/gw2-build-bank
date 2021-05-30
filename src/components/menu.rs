use maud::{html, Markup};

pub fn menu(page_title: &str) -> Markup {
  html! {
    div.menu style="-webkit-app-region: drag" {
      (menu_link(&html! {
        "home"
      }, "/", page_title, "home"))

      (menu_link(&html! {
        "add build"
      }, "/build/create", page_title, "add build"))

      (menu_link(&html! {
        "remove build"
      }, "/build/remove", page_title, "remove build"))

      img.background src="/assets/Bottom_Graphics_10.png";

      img.pattern_1 src="/assets/pattern3.jpg";
      img.pattern_2 src="/assets/pattern5.jpg";
    }
  }
}

fn menu_link(text: &Markup, href: &str, page_title: &str, match_str: &str) -> Markup {
  html! {
    @if page_title == match_str {
      a class="active" href=(href) { (text) }
    } @else {
      a href=(href) { (text) }
    }
  }
}