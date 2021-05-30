use maud::{html, Markup};

pub fn header(page_title: &str) -> Markup {
  // let css_path = format!("/static/{}.css", page_title);

  html! {
    head {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1.0";
      meta http-equiv="X-UA-Compatible" content="ie=edge";

      // the style is hardcoded because it allows me to ship a single binary
      // without any other files around it.
      style type="text/css" {
        (master_css_content())
      }
  
      title { (page_title) }
    }
  }
}

fn master_css_content() -> String {
  "
  html, body {
    display: flex;
    flex-direction: column;
    font-family: Bahnschrift;
    height: 100vh;
    overflow-y: hidden;
  
    padding: 0;
    margin: 0;

    color: #5f615c;
  }

  #content {
    overflow-y: auto;
    margin-top: 3em;
    flex-grow: 1;
  }


  .menu {
    display: flex;
    padding: 1em;
    color: rgb(229, 212, 166);
    position: relative;
  }
  .menu a {
    text-decoration: none;
    color: rgb(229, 212, 166);
  }
  .menu a + a {
    margin-left: 1em;
  }
  .menu a.active {
    text-decoration: underline;
  }
  .menu .background {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 200%;
    transform: rotate(180deg);
    z-index: -1;
  }

  .menu .pattern_1 {
    position: fixed;
    top: 0%;
    right: -25%;
    z-index: -2;
    opacity: .06;
    height: 50%;
  }

  .menu .pattern_2 {
    position: fixed;
    left: -10%;
    bottom: -50%;
    z-index: -2;
    opacity: .06;
    transform: rotate(70deg) translate(0%, 0%);
    width: 110%;
    height: 110%;
  }

  
  ".to_owned()
}