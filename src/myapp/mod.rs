use actix_web::{App};
mod root;
pub fn init() -> App {
    App::new()
        .resource("/", |r| r.f(root::index))
}