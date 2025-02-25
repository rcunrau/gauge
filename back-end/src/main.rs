mod device;

use device::{scan, temp};

#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/data", routes![scan, temp])
        .mount("/img", FileServer::from(relative!("img")).rank(2))
        .mount("/", FileServer::from(relative!("dist")))
}
