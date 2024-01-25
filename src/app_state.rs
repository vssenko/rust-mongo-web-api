use crate::injector::Injector;
use actix_web::{error, HttpResponse};

#[derive(Debug)]
pub struct AppState {
    pub i: Injector,
}

impl AppState {
    pub fn format_err(&self, e: error::Error) -> HttpResponse {
        return HttpResponse::from_error(e);
    }
}
