use actix_web::App;
/* 

we import our applications from our super which is the main file

*/
use super::myapp;

/**
 * get a vector of available applications
 * @return Vec<App> a vector of applications
 */
pub fn get() -> Vec<App> {
    vec![
        myapp::init(),
    ]
}

