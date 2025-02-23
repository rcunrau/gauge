mod gauge;

use gloo_net::http::Request;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_hooks::use_interval;

use log::{info, warn};

use gauge::{new_gauge, update_tach};

#[styled_component]
fn App() -> Html {
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
                   width: 30%;
                   flex: 1;
                   padding-left: 10px;
                   margin-left: 10px;
                   float: right;
                   background-color: #6699cc;
               }
            "#)} />

            <nav>
                <p>{ "Scan" }</p>
            </nav>
            <aside>
                <h2>{ "Devices" }</h2>
            </aside>
            <Place />
        </>
    }
}


#[function_component(Place)]
fn placel() -> Html {
    let window = web_sys::window().unwrap();

    let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
        info!("x,y: {},{}", e.offset_x(), e.offset_y());

        let svg = match new_gauge("g1", e.offset_x(), e.offset_y(), 100) {
            Ok(s) => s,
            Err(js) => {
                warn!("new_gauge {:?}", js);
                return;
            },
        };

        let document = web_sys::window().unwrap().document().unwrap();
        document.body().unwrap().append_child(&svg).unwrap();
    });

    window.add_event_listener_with_callback("mousedown", cb.as_ref().unchecked_ref()).unwrap();

    cb.forget();

    html!{
        <>
            <p>{"place component"}</p>
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
