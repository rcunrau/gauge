#[macro_use] extern crate rocket;

use rocket::fs::{relative, FileServer};
use std::sync::RwLock;

#[derive(Clone, Copy, Debug)]
struct State {
    val: i32,
    step: i32,
}

static STATE: RwLock<State> = RwLock::new(State { val: 50, step: 2 });

#[get("/rpm")]
fn rpm() -> String {
    let mut current = STATE.write().unwrap();
    if current.val >= 100 {
        current.val = 98;
        current.step = -2;
    }
    else if current.val <= 0 {
        current.val = 2;
        current.step = 2;
    }
    current.val += current.step;
    format!("{}", current.val)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/data", routes![rpm])
        .mount("/img", FileServer::from(relative!("img")).rank(2))
        .mount("/", FileServer::from(relative!("dist")))
}
