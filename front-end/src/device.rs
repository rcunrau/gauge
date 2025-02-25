use std::fmt;
use serde::{Deserialize, Serialize};
use yew::{html, Html};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DeviceMode {
    Off,
    Auto,
    On,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    model: Product,
    pub temp: i32,
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

    pub fn render(&self) -> Html {
        html! {
            <div class="card">
                <div class="container">
                    <h4 align="center">{ &self.name }</h4>
                    <p>{ "Mode: Auto" }</p>
                    <p align="center">{ format!("({})", self.model) }</p>
                </div>
            </div>
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum State {
    Init,
    Placing,
    Running,
}

impl State {
    pub fn render(&self) -> Html {
        html!{
            <p><b>{"State:"}</b>{ format!("{}", self) }</p>
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            State::Init => "Initializing",
            State::Placing => "Placing Devices",
            State::Running => "Running",
        };
        write!(f, "{}", s)
    }
}
