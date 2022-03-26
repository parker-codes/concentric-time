#![allow(non_snake_case)]

use dioxus::prelude::*;
use web_sys::console;

fn main() {
    console::log_1(&"Hello, world!".into());
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    // let current_time = use_state(&cx, || 0);
    // TODO: derive year, month, week, day, hour, minute, second from current_time

    cx.render(rsx! {
        Ring { size: 80.0, stroke_width: 8.5, color: RingColor::Blue }
        Ring { size: 400.0 }
    })
}

#[derive(PartialEq, Clone)]
enum RingColor {
    Gray,
    Red,
    Green,
    Blue,
}
impl Default for RingColor {
    fn default() -> Self {
        RingColor::Gray
    }
}
impl ToString for RingColor {
    fn to_string(&self) -> String {
        match self {
            RingColor::Gray => String::from("stroke-gray-500"),
            RingColor::Red => String::from("stroke-red-500"),
            RingColor::Green => String::from("stroke-green-500"),
            RingColor::Blue => String::from("stroke-blue-500"),
        }
    }
}

#[inline_props]
fn Ring(cx: Scope, size: f32, stroke_width: Option<f32>, color: Option<RingColor>) -> Element {
    let half = size / 2.0;
    let stroke_width = stroke_width.unwrap_or(4.0);
    let radius = half - stroke_width * 2.0;
    let stroke_color = color.clone().unwrap_or_default().to_string();

    console::log_1(&format!("size: {}", size).into());
    console::log_1(&format!("half: {}", half).into());
    console::log_1(&format!("stroke_width: {}", stroke_width).into());
    console::log_1(&format!("radius: {}", radius).into());
    console::log_1(&format!("stroke_color: {}", stroke_color).into());

    cx.render(rsx! {
        svg {
            width: "{size}",
            height: "{size}",

            circle {
                class: "{stroke_color}",
                stroke_width: "{stroke_width}",
                fill: "transparent",
                r: "{radius}",
                cx: "{half}",
                cy: "{half}"
            }
        }
    })
}
