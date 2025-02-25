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
    static ref STATES: Mutex<Vec<TempState>> = Mutex::new(
        vec![TempState { val: 40, step: 1, min: 10, max: 90 },
             TempState { val: 50, step: 2, min: 40, max: 90 },
             TempState { val: 75, step: 3, min: 50, max: 100 },
             TempState { val: 50, step: 4, min: 25, max: 75 },
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

#[derive(Clone, Copy, Debug)]
struct TempState {
    val: i32,
    step: i32,
    min: i32,
    max: i32,
}

impl TempState {
    fn update(&mut self) -> i32 {
        if self.val >= self.max {
            self.val = self.max;
            self.step = -self.step;
        }
        else if self.val <= self.min {
            self.val = self.min;
            self.step = -self.step;
        }
        self.val += self.step;

        self.val
    }
}

#[get("/temp")]
pub fn temp() -> Json<Vec<Device>> {
    let mut known_devices = REFINERY.lock().unwrap();
    let mut states = STATES.lock().unwrap();

    for (i, device) in known_devices.iter_mut().enumerate() {
        device.temp = states[i].update();
    }

    Json((*known_devices.clone()).to_vec())
}
