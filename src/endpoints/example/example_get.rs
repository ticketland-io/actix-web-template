use actix_web::{web, HttpResponse};
use eyre::Result;
use actix_middleware::firebase_auth::AuthData;
use crate::{
  utils::{store::Store, error::Error},
};

/// Example of a get request
pub async fn exec(
  _store: web::Data<Store>,
  _auth: AuthData,
) -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().finish())
}
