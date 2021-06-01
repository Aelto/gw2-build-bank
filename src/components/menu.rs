use maud::{html, Markup};

pub fn menu(page_title: &str) -> Markup {
  html! {
    div.menu.drag-region {
      div.left.drag-region {
        (menu_link(&html! {
          "builds"
        }, "/", page_title, "home"))
  
        (menu_link(&html! {
          "add"
        }, "/build/create", page_title, "add build"))
  
        (menu_link(&html! {
          "remove"
        }, "/build/remove", page_title, "remove build"))

        (menu_link(&html! {
          "edit"
        }, "/build/edit", page_title, "edit build"))
      }
      div.right.drag-region {
        a id="minimize" { "_" }
        a id="close" { "x" }
      }

      img.background src="/assets/Bottom_Graphics_10.png";
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