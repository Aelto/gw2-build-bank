use crate::models::build::{Build};

use serde::{Serialize, Deserialize};
use actix_web::{web, HttpRequest, HttpResponse, Result, http};


#[derive(Serialize, Deserialize)]
pub struct CreateBuildBody {
  pub build_template: String,
  pub build_name: String,
  pub build_description: String,
  pub build_profession: String
}

pub async fn create_build(_req: HttpRequest, form: web::Form<CreateBuildBody>) -> Result<HttpResponse> {
  let build = Build::new(
    &form.build_template,
    &form.build_name,
    &form.build_description,
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

#[derive(Serialize, Deserialize)]
pub struct GetBuildBody {
  pub build_name: String
}

pub async fn get_build(req: HttpRequest) -> Result<HttpResponse> {
  let build_name = req
  .match_info()
  .get("build_name")
  .unwrap_or_else(|| "__unknown__");

  let build = Build::by_name(build_name)
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("Internal server error: could not find the build. {}", err))
  })?;

  Ok(
    HttpResponse::Ok()
    .json(build)
  )
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBuildBody {
  pub build_template: String,
  pub build_name: String,
  pub build_description: String,
  pub build_profession: String
}

pub async fn update_build(_req: HttpRequest, form: web::Form<UpdateBuildBody>) -> Result<HttpResponse> {
  let build = Build::by_name(&form.build_name)
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("Internal server error: could not get the build. {}", err))
  })?;

  if build.is_none() {
    return Ok(
      HttpResponse::InternalServerError()
      .content_type("text/plain")
      .body(format!("Internal server error: no such build. {}", form.build_name))
    );
  }

  let mut build = build.unwrap();

  build.profession = form.build_profession.to_owned();
  build.template = form.build_template.to_owned();
  build.description = form.build_description.to_owned();

  dbg!(&build.profession);

  build.update()
  .map_err(|err| {
    HttpResponse::InternalServerError()
    .content_type("text/plain")
    .body(format!("Internal server error: could not update the build. {}", err))
  })?;

  Ok(
    HttpResponse::Found()
    .header(http::header::LOCATION, "/build/edit")
    .content_type("text/plain")
    .body("removed")
  )
}