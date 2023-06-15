use std::{
  io::Result, rc::Rc, env, panic, process,
};
use actix_cors::Cors;
use actix_web::{middleware, web, http, App, HttpServer, HttpResponse};
use env_logger::Env;
use actix_middleware::firebase_auth::AuthnMiddlewareFactory;
use actix_web_api::{
  utils::store::Store,
  endpoints::{
    example::config::config as example_config,
  },
};

#[actix_web::main]
async fn main() -> Result<()> {
  let orig_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info| {
    orig_hook(panic_info);
    process::exit(1);
  }));

  if env::var("ENV").unwrap() == "development" {
    dotenv::from_filename(".env").expect("cannot load env from a file");
  }

  let store = Store::new().await;
  let port = store.config.port;
  let cors_origin = store.config.cors_config.origin.clone();
  let firebase_api_key = store.config.firebase_api_key.clone();
  let store = web::Data::new(store);

  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

  HttpServer::new(move || {
    let authn_middleware = Rc::new(AuthnMiddlewareFactory::new(firebase_api_key.as_ref().unwrap().to_owned()));
    let cors_origin = cors_origin.clone();

    let cors = Cors::default()
    .allowed_origin_fn(move |origin, _| {
      cors_origin.iter().any(|v| v == origin || v == "*")
    })
    .allowed_methods(vec!["GET", "POST", "PUT"])
    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    .allowed_header(http::header::CONTENT_TYPE)
    .max_age(3600);

    App::new()
      .app_data(store.clone())
      .wrap(cors)
      .wrap(middleware::Logger::default())
      .service(web::scope("/auth").configure(example_config(Rc::clone(&authn_middleware))))
      .route("/", web::get().to(|| HttpResponse::Ok()))
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
