#![feature(proc_macro_hygiene)]

// uncomment the line below when building a release.
// It allows the binary to start in background without a cli window.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ops::Add, sync::{Arc, Mutex, mpsc}, thread, time::{Duration, Instant}};
use actix_web::{App, web, HttpServer};
use wry::application::{event, event_loop::EventLoopWindowTarget, platform::windows::WindowBuilderExtWindows};

mod pages;
mod components;
mod constants;
mod api;
mod models;
mod utils;
mod asset_handler;

#[actix_rt::main]
async fn main() -> wry::Result<()> {
  let (server_tx, server_rx) = mpsc::channel();
  let (port_tx, port_rx) = mpsc::channel();

  // start actix web server in separate thread
  thread::spawn(move || {
    let sys = actix_rt::System::new("guild-bank-actix");

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
      .service(web::resource("/build/edit").route(web::get().to(pages::edit_build::render)))
  
      // api endpoints
      .service(
        web::scope("/api")
          .route("/program/ping", web::post().to(api::program::ping))
          .route("/program/exit", web::post().to(api::program::exit))
          .route("/program/clipboard", web::post().to(api::program::clipboard))
          .route("/build/create", web::post().to(api::build::create_build))
          .route("/build/remove", web::post().to(api::build::remove_build))
          .route("/build/update", web::post().to(api::build::update_build))
          .route("/build/{build_name}", web::get().to(api::build::get_build))
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

  use wry::{
    application::{
      event::{Event, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::{Window, WindowBuilder, Icon},
      dpi::PhysicalSize,
    },
    webview::{RpcRequest, WebViewBuilder},
  };

  let event_loop = EventLoop::new();
  let mut webviews = std::collections::HashMap::new();
  let window = WindowBuilder::new()
    .with_inner_size(PhysicalSize::new(350, 450))
    .with_decorations(false)
    .with_transparent(true)
    .with_title("GW2 Build bank")
    .with_taskbar_icon(Some(asset_handler::get_icon()))
    .build(&event_loop)
    .unwrap();

  let (window_tx, window_rx) = std::sync::mpsc::channel();

  let handler = move |window: &Window, req: RpcRequest| {
    if req.method == "minimize" {
      window.set_minimized(true);
    }
    if req.method == "maximize" {
      if window.is_maximized() {
        window.set_maximized(false);
      } else {
        window.set_maximized(true);
      }
    }
    if req.method == "close" {
      let _ = window_tx.send(window.id());

      // weird case where the event is not captured instantly until a new event
      // is triggered, so we minimize the window to trigger it twice.
      window.set_minimized(true);

    }
    if req.method == "drag_window" {
      let _ = window.drag_window();
    }
    None
  };

  let webview = WebViewBuilder::new(window)
    .unwrap()
    .with_url(&format!("http://127.0.0.1:{}/", port))?
    .with_initialization_script(&get_drag_script())
    .with_transparent(true)
    .with_rpc_handler(handler)
    .build()?;

  let id = webview.window().id();
  webviews.insert(webview.window().id(), webview);

  let (keyboard_tx, keyboard_rx) = std::sync::mpsc::sync_channel(2);


  inputbot::KeybdKey::MKey.bind(move || {
    dbg!("yes");
    if inputbot::KeybdKey::LControlKey.is_pressed() {
      dbg!("2");
      keyboard_tx.send(id);
    }
  });

  inputbot::handle_input_events();


  event_loop.run(move |event, window_target, control_flow| {
    *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1));

    if let Ok(id) = window_rx.try_recv() {
      webviews.remove(&id);
      if webviews.is_empty() {
        *control_flow = ControlFlow::Exit
      }
    }

    if let Ok(id) = keyboard_rx.try_recv() {
      if let Some(window) = webviews.get(&id) {
        let window = window.window();
        window.set_minimized(false);
        window.set_focus();
      }
    }

    if let Event::WindowEvent { event, window_id } = event {
      match event {
        WindowEvent::CloseRequested => {
          webviews.remove(&window_id);
          if webviews.is_empty() {
            *control_flow = ControlFlow::Exit
          }
        }
        WindowEvent::Resized(_) => {
          let _ = webviews[&window_id].resize();
        }
        _ => (),
      }
    }
  });

  // gracefully shutdown actix web server
  server.stop(true).await;

  Ok(())
}

fn get_drag_script() -> String {
  let script = r#"
  (function () {
    window.addEventListener('DOMContentLoaded', (event) => {
      document.getElementById('minimize').addEventListener('click', () => rpc.notify('minimize'));
      //document.getElementById('maximize').addEventListener('click', () => rpc.notify('maximize'));
      document.getElementById('close').addEventListener('click', () => rpc.notify('close'));
      document.addEventListener('mousedown', (e) => {
        if (e.target.classList.contains('drag-region') && e.buttons === 1) {
          window.rpc.notify('drag_window');
        }
      })
    });
  })();
  "#;

  script.to_string()
}