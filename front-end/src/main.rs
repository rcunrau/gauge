use yew::prelude::*;
use gloo_net::http::Request;
use yew_hooks::use_interval;
use log::{info, warn};
use std::f64::consts::PI;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Tachometer" }</h1>
            <div>
            <Tachometer width={500} height={400} />  
            </div>
            <Poll />
       </>
    }
}

#[function_component(Periodic)]
fn periodic() -> Html {
    let counter = use_state(|| 1);

    {
        let num = counter.clone();
	    use_interval(move || {
            update_tach(format!("{}", *num));
	        if *num >= 100 {
	            num.set(1);
	        }
	        else {
                num.set(*num + 1);
	        }
	        info!("interval {}", *num);
   	    }, 1000);
    }

    html! {
        <div></div>
    }
}

#[function_component(Poll)]
fn poll() -> Html {
	use_interval(move || {
        wasm_bindgen_futures::spawn_local(async move {
            let response = Request::get("data/rpm")
                .send().await.unwrap()
                .text().await.unwrap();

            update_tach(response);
        });
   	}, 1000);

    html! {
        <div></div>
    }
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

fn update_tach(text: String) {
    let percent = match text.parse::<u32>() {
        Ok(val) => val,
        Err(e) => {
            warn!("can't parse <{}>: {}", text, e);
            return;
        },
    };
    let (px, py) = plot_percent((percent as f64)/100.0);
    let document = web_sys::window()
        .unwrap()
	.document()
	.unwrap();
    let pointer = document.get_element_by_id("line")
	.unwrap();
    pointer.set_attribute("x2", &format!("{}", px)).unwrap();
    pointer.set_attribute("y2", &format!("{}", py)).unwrap();
    info!("px: {}, py: {}", px, py);
}

const RADIUS: f64 = 150.0;
const XORIGIN: f64 = 250.0;
const YORIGIN: f64 = 200.0;

fn plot_percent(percent: f64) -> (f64, f64) {
    let radians = percent*PI;
    let unitx = radians.cos();
    let unity = radians.sin();

    (XORIGIN - unitx * RADIUS, YORIGIN - unity * RADIUS)
}
        
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    
    yew::Renderer::<App>::new().render();
}
