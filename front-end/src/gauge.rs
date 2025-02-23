use wasm_bindgen::prelude::*;
use web_sys::SvgElement;

use log::{info, warn};
use std::f64::consts::PI;

pub fn new_gauge(name: &str, x: i32, y: i32, size: u32) -> Result<SvgElement, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .unwrap()
        .dyn_into::<web_sys::SvgElement>()
        .map_err(|_| ())
        .unwrap();

    let half = format!("{}px", size/2);
    let third = format!("{}px", size/3);
    let left = x - (size as i32)/2;
    let top = y - (size as i32)/2;
    // svg.style().set_property("border", "1px solid black")?;
    svg.style().set_property("position", "absolute")?;
    svg.style().set_property("left", &left.to_string())?;
    svg.style().set_property("top", &top.to_string())?;
    
    svg.set_attribute("width", &size.to_string())?;
    svg.set_attribute("height", &size.to_string())?;

    // Clip rectangle
    let clip = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "clipPath")?;
    clip.set_attribute("id", "cut-bottom")?;
    let rect = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "rect")?;
    rect.set_attribute("x", "0")?;
    rect.set_attribute("y", "0")?;
    rect.set_attribute("width", &size.to_string())?;
    rect.set_attribute("height", &half)?;
    clip.append_child(&rect)?;
    svg.append_child(&clip)?;

    // Top of the gauge
    let top_arc = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    top_arc.set_attribute("cx", &half)?;
    top_arc.set_attribute("cy", &half)?;
    top_arc.set_attribute("r", &half)?;
    top_arc.set_attribute("stroke", "black")?;
    top_arc.set_attribute("fill", "red")?;
    top_arc.set_attribute("clip-path", "url(#cut-bottom)")?;
    svg.append_child(&top_arc)?;

    // Bottom of the gauge
    let bot_arc = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle")?;
    bot_arc.set_attribute("cx", &half)?;
    bot_arc.set_attribute("cy", &half)?;
    bot_arc.set_attribute("r", &third)?;
    bot_arc.set_attribute("stroke", "black")?;
    bot_arc.set_attribute("fill", "yellow")?;
    bot_arc.set_attribute("clip-path", "url(#cut-bottom)")?;
    svg.append_child(&bot_arc)?;

    // Needle
    let line = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;
    line.set_attribute("x1", &half)?;
    line.set_attribute("y1", &half)?;
    line.set_attribute("x2", &half)?;
    line.set_attribute("y2", "0")?;
    line.set_attribute("stroke", "black")?;
    line.set_attribute("stroke-width", "3")?;
    line.set_attribute("id", name)?;
    svg.append_child(&line)?;

    Ok(svg)
}

pub fn update_tach(name: &str, text: String) {
    let percent = match text.parse::<u32>() {
        Ok(val) => val,
        Err(e) => {
            warn!("can't parse <{}>: {}", text, e);
            return;
        },
    };
    let (px, py) = plot_percent(250.0, 200.0, 150.0, (percent as f64)/100.0);

    let document = web_sys::window().unwrap().document().unwrap();
    let pointer = match document.get_element_by_id(name) {
        Some(p) => p,
        None => {
            warn!("pointer {} not found", name);
            return;
        },
    };
    pointer.set_attribute("x2", &format!("{}", px)).unwrap();
    pointer.set_attribute("y2", &format!("{}", py)).unwrap();
    info!("px: {}, py: {}", px, py);
}

fn plot_percent(cx: f64, cy: f64, radius: f64, percent: f64) -> (f64, f64) {
    let radians = percent*PI;
    let unitx = radians.cos();
    let unity = radians.sin();

    (cx - unitx * radius, cy - unity * radius)
}
        
