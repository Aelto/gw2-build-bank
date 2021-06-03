use maud::{html, Markup};
use crate::models::build::Build;

pub fn build_list(builds: Vec<Build>) -> Markup {
  html! {
    ul class="build-list" {
      @for build in builds {
        li onclick={"setClipboard('" (build.template) "', event)"} title=(build.description) {
          img src={"/assets/icons/"(build.profession)".png"};
          (build.name)
        }
      }
    }

    script { (maud::PreEscaped("
      async function setClipboard(text, e) {
        navigator.clipboard.writeText(text);
        console.log(e);
        
        e.target.classList.remove('copied');
        await new Promise(resolve => setTimeout(resolve, 32));
        e.target.classList.add('copied');
      }
    ")) }

    style type="text/css" { "
      @keyframes slidein {
        0% {
          transform: translate(-175%, -50%);
          opacity: 0;
        }
      
        25% {
          transform: translate(-200%, -50%);
          opacity: 1;
        }

        90% {
          transform: translate(-200%, -50%);
          opacity: 1;
        }
        100% {
          opacity: 0;
        }
      }

      .build-list {

      }
      .build-list li {
        list-style: none;
        position: relative;
        margin-top: .5em;
        font-size: 1.2em;
        cursor: pointer;
        position: relative;
      }
      .build-list li.copied:before {
        content: 'copied';
        position: absolute;
        left: 0;
        top: 50%;
        transform: translate(-200%, -50%);
        z-index: 40;
        background: #b7b7b7;
        padding: 3px;
        border-radius: 6px;
        color: white;
        font-size: 12px;
        box-shadow: 0px 2px 12px rgb(20 20 20 / 20%);
        opacity: 0;
        animation-duration: 1s;
        animation-name: slidein;
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
        filter: drop-shadow(0px 1px 0px rgba(20, 20, 20, .9));
        transition: .25s all ease-in-out;
      }
    " }
  }
}

fn profession_icon(profession: &str) -> Markup {
  html! {
    span class=(profession);
  }
}