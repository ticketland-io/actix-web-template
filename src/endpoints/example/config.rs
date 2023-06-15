use std::{rc::Rc};
use actix_web::{web};
use actix_middleware::firebase_auth::AuthnMiddlewareFactory;
use super::example_get;

/// Example of specific resource endpoint configuration
pub fn config(authn_middleware: Rc<AuthnMiddlewareFactory>) -> impl FnOnce(&mut web::ServiceConfig) {
  move |cfg: &mut web::ServiceConfig| {
    cfg.service(
      web::resource("")
      .wrap(Rc::clone(&authn_middleware))
      .route(web::get().to(example_get::exec))
    );
  }
}
