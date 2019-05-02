extern crate actix_web;

use actix_web::server;

/*

we import `applications` to pass to our server
we import our applications on here first so we can use them
later on `applications` so we can initialize them

*/
mod applications;
mod myapp;

fn main() {
    server::new(|| {
        applications::get()
    }).bind("127.0.0.1:7522").unwrap().run();
}
