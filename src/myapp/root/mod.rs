use actix_web::{HttpRequest, Responder};

/*
    these handlers will are highly versatile
    checkout https://actix.rs/ for docs
*/
pub fn index(_: &HttpRequest) -> impl Responder {
    "Hello world!"
}