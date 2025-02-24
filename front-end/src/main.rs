mod gauge;
mod device;

use gloo_net::http::Request;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew_hooks::use_interval;

use log::{info, warn};

use gauge::{new_gauge, update_tach};
use device::Device;

#[derive(Clone, PartialEq)]
enum State {
    Init,
    Placing,
    Running,
}

#[styled_component]
fn App() -> Html {
    let state = use_state(|| State::Init);
    let devices = use_state(Vec::new);

    let scan = {
        let devices = devices.clone();
        let state = state.clone();
        Callback::from(move |_| {
            let devices = devices.clone();
            let state = state.clone();
            spawn_local(async move {
                match Request::get("data/scan").send().await {
                    Ok(response) => {
                        let scanned: Vec<Device> = response.json().await.unwrap();
                        devices.set(scanned);

                        state.set(State::Placing);
                    },
                    Err(e) => warn!("Couldn't fetch users: {:?}", e),
                }
            });
        })
    };

    let cb = {
        let state = state.clone();
        Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
            let state = state.clone();

            if *state != State::Placing {
                return;
            }

            info!("x,y: {},{}", e.offset_x(), e.offset_y());

            let svg = match new_gauge("g1", e.offset_x(), e.offset_y(), 150) {
                Ok(s) => s,
                Err(js) => {
                    warn!("new_gauge {:?}", js);
                    return;
                },
            };

            let document = web_sys::window().unwrap().document().unwrap();
            document.body().unwrap().append_child(&svg).unwrap();
        })
    };

    let window = web_sys::window().unwrap();
    window.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref()).unwrap();

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
            </nav>
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
            let response = Request::get("data/rpm")
                .send().await.unwrap()
                .text().await.unwrap();

            update_tach("line", response);
        });
   	}, 1000);

    html! {}
}

#[derive(Properties, PartialEq)]
struct TachProperties {
    width: u32,
    height: u32,
}

#[function_component(Tachometer)]
fn tachometer(props: &TachProperties) -> Html {
    let w = format!("{}", props.width);
    let h = format!("{}", props.height);
    let halfw = format!("{}", props.width/2);
    let halfh = format!("{}", props.height/2);
    let thirdh = format!("{}", props.height/3);
    html! {
        <svg width={w.clone()} height={h.clone()} xmlns="http://www.w3.org/2000/svg">
            <defs>
                <@{"clipPath"} id="cut-bottom">
                    <rect x="0" y="0" width={w.clone()} height={halfh.clone()} />
                </@>
                <marker id="arrow" markerWidth="10" markerHeight="10"
                    refX="5" refY="5" orient="auto">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="black" />
                </marker>
            </defs>
            <circle cx={halfw.clone()} cy={halfh.clone()} r={halfh.clone()}
                fill="red" clip-path="url(#cut-bottom)" />
            <circle cx={halfw.clone()} cy={halfh.clone()} r={thirdh.clone()}
                fill="yellow" clip-path="url(#cut-bottom)" />
            <line id="line" x1={halfw.clone()} y1={halfh.clone()} x2={halfw.clone()} y2="60" stroke="black" stroke-width="5"
                marker-end="url(#arrow)" />
            { "Sorry, your browser does not support inline SVG." }
        </svg>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    yew::Renderer::<App>::new().render();
}
