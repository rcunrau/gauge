mod gauge;
mod device;

use gloo_net::http::Request;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew_hooks::use_interval;

use log::{info, warn};

use gauge::{new_gauge, update_gauge, GAUGE_SIZE};
use device::{Device, State};

#[styled_component]
fn App() -> Html {
    let state = use_state(|| State::Init);
    let devices = use_state(Vec::new);

    let scan = {
        let state = state.clone();
        let devices = devices.clone();
        Callback::from(move |_| {
            let devices = devices.clone();
            let state = state.clone();
            spawn_local(async move {
                match Request::get("data/scan").send().await {
                    Ok(response) => {
                        let scanned: Vec<Device> = response.json().await.unwrap();
                        state.set(State::Placing);
                        devices.set(scanned);
                    },
                    Err(e) => warn!("Couldn't fetch users: {:?}", e),
                }
            });
        })
    };

    let cb = {
        let state = state.clone();
        let devices = devices.clone();
        Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
            if *state != State::Placing {
                return;
            }

            info!("x,y: {},{}", e.offset_x(), e.offset_y());

            let document = web_sys::window().unwrap().document().unwrap();
            for device in devices.iter() {
                match document.get_element_by_id(&device.name) {
                    Some(_) => continue,
                    None => match new_gauge(&device.name, e.offset_x(), e.offset_y(), GAUGE_SIZE) {
                        Ok(svg) => {
                            document.body().unwrap().append_child(&svg).unwrap();
                            return;
                        }
                        Err(js) => {
                                warn!("new_gauge {:?}", js);
                                state.set(State::Running);  // Don't try to place any more
                                return;
                        },
                    },
                }
            }

            info!("moving to state running!");
            state.set(State::Running);  // All devices placed
        })
    };

    let window = web_sys::window().unwrap();
    window.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();

    info!("window w: {:?}, h: {:?}", window.inner_width().unwrap().as_f64(),
          window.inner_height().unwrap().as_f64());
    cb.forget();

    let device_list = devices.clone().iter().map(|d| d.render()).collect::<Html>();
        
    html! {
        <>
            <Global css={css!(r#"
               html, body {
                   background-image: url("img/gettyimages-472321657-612x612.jpg");
               }
               nav, aside {
                   background-color: white;
                   padding: 1%;
               }

               nav {
                   height: 50px;
                   background-color: #6699cc;
                   display: flex;
                   margin-bottom: 10px;
               }

               aside {
                   width: 10%;
                   flex: 1;
                   padding-left: 10px;
                   margin-left: 10px;
                   float: right;
                   background-color: #6699cc;
               }

               .card {
                   /* Add shadows to create the "card" effect */
                   box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
                   transition: 0.3s;
                   background-color: lightblue;
               }

               /* On mouse-over, add a deeper shadow */
               .card:hover {
                   box-shadow: 0 8px 16px 0 rgba(0,0,0,0.2);
               }

               /* Add some padding inside the card container */
               .container {
                   padding: 2px 16px;
               }
            "#)} />

            <nav>
                <button onclick={scan}>{ "Scan" }</button>
                { state.clone().render() }
            </nav>
            <Poll />
            <aside>
                <h2>{ "Devices" }</h2>
                { device_list }
            </aside>
        </>
    }
}

#[function_component(Poll)]
fn poll() -> Html {
	use_interval(move || {
        wasm_bindgen_futures::spawn_local(async move {
            match Request::get("data/temp").send().await {
                Ok(response) => {
                    let temps: Vec<Device> = response.json().await.unwrap();

                    for dev in temps {
                        update_gauge(&dev.name, &dev.temp.to_string());
                    }
                },
                Err(e) => warn!("update failed: {}", e),
            };
        });
    }, 400);

    html! {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    yew::Renderer::<App>::new().render();
}
