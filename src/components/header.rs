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
    height: calc(100vh - 2em);
    overflow-y: hidden;
  
    padding: 0;
    margin: 0;

    color: #5f615c;
  }

  html {
    padding: 1em;
  }

  body {
    position: relative;
    box-shadow: 0px 0px 12px -5px rgb(20 20 20 / 80%);
    border-radius: 6px;
  }

  #content {
    overflow-y: auto;
    flex-grow: 1;
    padding: 0 3em;
    padding-top: 1.3em;
  }


  .menu {
    display: flex;
    padding: 1em;
    color: rgb(229, 212, 166);
    position: relative;
  }
  .menu .left {
    flex-grow: 1;
  }
  .menu .left, .menu .right {
    display: flex;
  }
  .menu .right a {
    padding: 4px;
  }
  .menu .right a:hover {
    filter: brightness(1.5);
  }
  .menu a {
    text-decoration: none;
    color: rgb(229, 212, 166);
    cursor: pointer;
  }
  .menu a + a {
    margin-left: 1em;
  }
  .menu a.active {
    text-decoration: underline;
  }
  .menu .background {
    position: absolute;
    top: -2px;
    left: 0;
    width: 100%;
    height: 200%;
    transform: rotate(180deg);
    z-index: -1;
    filter: drop-shadow(2px 4px 6px rgba(20, 20, 20, .4));
  }
  
  #background {
    position: absolute;
    top: 10px;
    left: 0;
    width: 100%;
    height: 100%;
    background: white;
    z-index: -2;
    overflow: hidden;
  }
  .pattern_1 {
    position: absolute;
    top: 0%;
    right: -25%;
    z-index: -2;
    opacity: .06;
    height: 50%;
  }

  .pattern_2 {
    position: absolute;
    left: -10%;
    bottom: -50%;
    z-index: -2;
    opacity: .06;
    transform: rotate(70deg) translate(0%, 0%);
    width: 110%;
    height: 110%;
  }

  .pattern_3 {
    position: absolute;
    left: -6%;
    bottom: -28%;
    z-index: -3;
    opacity: 0.2;
    width: 110%;
    height: 110%;
    transform: rotateY(180deg);
    filter: grayscale(1);
  }

  label, input[type='submit'] {
    margin-top: 2em;
    display: block;
  }

  label, select {
    margin-top: 1em;
  }

  input, select, textarea {
    border: solid 1px;
    padding: 4px;
    background: rgba(255, 255, 255, .8);
    width: 100%;
    box-sizing: border-box;
    font-size: 1em;
    font-family: inherit;
  }

  option {
    font-size: 1.3em;
  }
  ".to_owned()
}