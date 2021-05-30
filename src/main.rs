#![feature(proc_macro_hygiene)]

// uncomment the line below when building a release.
// It allows the binary to start in background without a cli window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::mpsc, thread};
use actix_web::{App, web, HttpServer};
use web_view::Content;

mod pages;
mod components;
mod constants;
mod api;
mod models;
mod utils;
mod asset_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let (server_tx, server_rx) = mpsc::channel();
  let (port_tx, port_rx) = mpsc::channel();

  // start actix web server in separate thread
  thread::spawn(move || {
    let sys = actix_rt::System::new("actix-example");

    #[cfg(not(debug_assertions))]
    let address = "127.0.0.1:0";

    #[cfg(debug_assertions)]
    let address = "127.0.0.1:5000";

    let server = HttpServer::new(|| {
      App::new()
      .service(web::resource("/assets/{_:.*}").route(web::get().to(asset_handler::asset_handler)))
      // home page
      .service(web::resource("/").route(web::get().to(pages::root::render)))
      .service(web::resource("/build/create").route(web::get().to(pages::create_build::render)))
      .service(web::resource("/build/remove").route(web::get().to(pages::remove_build::render)))
  
      // api endpoints
      .service(
        web::scope("/api")
          .route("/program/ping", web::post().to(api::program::ping))
          .route("/program/exit", web::post().to(api::program::exit))
          .route("/program/clipboard", web::post().to(api::program::clipboard))
          .route("/build/create", web::post().to(api::build::create_build))
          .route("/build/remove", web::post().to(api::build::remove_build))
      )
  
    })
    .bind(address)
    .unwrap();

    // we specified the port to be 0,
    // meaning the operating system
    // will choose some available port
    // for us
    // get the first bound address' port,
    // so we know where to point webview at
    let port = server.addrs().first().unwrap().port();
    let server = server.run();

    let _ = port_tx.send(port);
    let _ = server_tx.send(server);
    let _ = sys.run();
  });

  let port = port_rx.recv().unwrap();
  let server = server_rx.recv().unwrap();

  dbg!(port);

  // start web view in current thread
  // and point it to a port that was bound
  // to actix web server
  let webview = web_view::builder()
      .title("GW2 build bank")
      .content(Content::Url(format!("http://127.0.0.1:{}/", port)))
      .size(350, 450)
      .resizable(true)
      .debug(true)
      .user_data(())
      .invoke_handler(|_webview, _arg| Ok(()))
      .run()
      .unwrap();

  // gracefully shutdown actix web server
  let _ = server.stop(true).await;

    Ok(())
}
