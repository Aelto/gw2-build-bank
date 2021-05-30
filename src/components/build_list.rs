use maud::{html, Markup};
use crate::models::build::Build;

pub fn build_list(builds: Vec<Build>) -> Markup {
  html! {
    ul class="build-list" {
      @for build in builds {
        li onclick={"setClipboard('" (build.template) "')"} {
          img src={"/assets/icons/"(build.profession)".png"};
          (build.name)
        }
      }
    }

    script { "
      function setClipboard(text) {
        const formData  = new FormData();
        formData.append('text', text);

        fetch('/api/program/clipboard', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'
          },
          body: `text=${text}`
        });
        
        // unfortunately doesn't work in the webview
        //navigator.clipboard.writeText(text);
      }
    " }

    style type="text/css" { "
      .build-list {

      }
      .build-list li {
        list-style: none;
        position: relative;
        margin-top: .5em;
        font-size: 1.2em;
        cursor: pointer;
      }
      .build-list li span {
        margin-right: 1em;
      }
      .build-list li span:before {
        content: '[';
      }
      .build-list li span:after {
        content: ']';
      }
      .build-list li span:before,
      .build-list li span:after {
        transform: translate(0px, 2px);
      }
      .build-list li:active {
        text-decoration: underline;
      }
      .build-list li:hover {
        text-decoration: underline;
      }
      .build-list li img {
        height: 24px;
        margin-right: .5em;
        position: absolute;
        left: -24px;
        top: 50%;
        transform: translate(-50%, -50%);
        filter: drop-shadow(0px 1px 5px rgba(20, 20, 20, 0.2));
      }
    " }
  }
}

fn profession_icon(profession: &str) -> Markup {
  html! {
    span class=(profession);
  }
}