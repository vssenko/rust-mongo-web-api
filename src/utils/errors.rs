use actix_web::error;
pub use actix_web::error::Error;

pub fn build_unauth_err() -> Error {
    error::ErrorUnauthorized("Not authorized")
}

pub fn build_generic_err() -> Error {
    error::ErrorInternalServerError("Internal error")
}
