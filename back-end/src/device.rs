use std::fmt;
use std::sync::Mutex;
use lazy_static::lazy_static;

use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

lazy_static! {
    static ref REFINERY: Mutex<Vec<Device>> = Mutex::new(
        vec![Device::new("Feed", Product::Kraken),
             Device::new("Intermediate", Product::NGC40),
             Device::new("Catalyst", Product::T1500),
             Device::new("Output", Product::Kraken),
        ]);
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum Product {
    NGC40 = 40,
    T1500 = 1500,
    Kraken = 8,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Product::NGC40 => "NGC-40",
            Product::T1500 => "T-1500",
            Product::Kraken => "Kraken",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum DeviceMode {
    Off,
    Auto,
    On,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Device {
    name: String,
    model: Product,
    temp: i32,
    mode: DeviceMode,
}

impl Device {
    pub fn new(name: &str, model: Product) -> Self {
        Self {
            name: name.to_string(),
            model,
            temp: 0,
            mode: DeviceMode::Off,
        }
    }
}

#[get("/scan")]
pub fn scan() -> Json<Vec<Device>> {
    let known_devices = REFINERY.lock().unwrap();

    Json((*known_devices.clone()).to_vec())
}
