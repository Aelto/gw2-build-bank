use actix_web::{HttpRequest, HttpResponse, Result, http, web};
use serde::{Serialize, Deserialize};


pub async fn exit(_req: HttpRequest) -> Result<HttpResponse> {
  std::process::exit(0);
}

pub async fn ping(_req: HttpRequest) -> Result<HttpResponse> {
  Ok(
    HttpResponse::Found()
      .content_type("text/plain")
      .body("")
  )
}

#[derive(Serialize, Deserialize)]
pub struct ClipboardBody {
  pub text: String
}

pub async fn clipboard(_req: HttpRequest, form: web::Form<ClipboardBody>) -> Result<HttpResponse> {
  use clipboard::ClipboardProvider;

  let mut clipboard_provider = clipboard::ClipboardContext::new()
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("could not open the clipboard. {}", err))
  })?;

  clipboard_provider.set_contents(form.text.to_string())
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("could not set the clipboard. {}", err))
  })?;

  Ok(
    HttpResponse::Found()
    .header(http::header::LOCATION, "/")
    .content_type("text/plain")
    .body("clipboard set")
  )
}
