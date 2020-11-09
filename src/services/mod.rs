use crate::errors::TalliiError;
use actix_web::HttpResponse;

pub mod auth;
pub mod events;
pub mod friends;
pub mod users;

type TalliiResponse = Result<HttpResponse, TalliiError>;
