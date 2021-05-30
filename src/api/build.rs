use crate::models::build::{Build};

use serde::{Serialize, Deserialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};


#[derive(Serialize, Deserialize)]
pub struct CreateBuildBody {
  pub build_template: String,
  pub build_name: String,
  pub build_profession: String
}

pub async fn create_build(_req: HttpRequest, form: web::Form<CreateBuildBody>) -> Result<HttpResponse> {
  let build = Build::new(
    &form.build_template,
    &form.build_name,
    &form.build_profession
  );

  let inserted = build.insert()
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("Internal server error: could not create the build. {}", err))
  })?;

  match inserted {
    true => Ok(
      HttpResponse::Found()
      .header(http::header::LOCATION, "/")
      .content_type("text/plain")
      .body("created")
    ),

    false => Ok(
      HttpResponse::Found()
      .header(http::header::LOCATION, "/")
      .content_type("text/plain")
      .body("already exists")
    )
  }

  
}

#[derive(Serialize, Deserialize)]
pub struct RemoveBuildBody {
  pub build_name: String
}

pub async fn remove_build(_req: HttpRequest, form: web::Form<RemoveBuildBody>) -> Result<HttpResponse> {
  Build::remove(&form.build_name)
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("Internal server error: could not remove the build. {}", err))
  })?;

  Ok(
    HttpResponse::Found()
    .header(http::header::LOCATION, "/")
    .content_type("text/plain")
    .body("removed")
  )
}