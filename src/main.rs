#![allow(non_snake_case)]

use dioxus::prelude::*;
use js_sys::Date;
use web_sys::console;

fn main() {
    console::log_1(&"Hello, world!".into());
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    let current_time = use_state(&cx, || Date::new_0());
    // let breakdown = get_breakdown(current_time.get());
    // let percentages = get_percentages(breakdown);

    use_future(&cx, current_time, move |current_time| {
        let mut interval = async_timer::Interval::platform_new(core::time::Duration::from_secs(1));
        async move {
            loop {
                interval.as_mut().await;
                current_time.set(Date::new_0());
                console::log_1(&format!("current_time: {}", current_time.to_iso_string()).into());
            }
        }
    });

    cx.render(rsx! {
        Ring { percent: 30.0, radius: 40.0, stroke: 8.5, color: RingColor::Blue }
        Ring { percent: 50.0, radius: 60.0, stroke: 6.5, color: RingColor::Green }
        Ring { percent: 75.0, radius: 80.0, stroke: 5.0, color: RingColor::Red }
        Ring { percent: 100.0, radius: 100.0 }
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
fn Ring(
    cx: Scope,
    percent: f32,
    radius: f32,
    color: Option<RingColor>,
    stroke: Option<f32>,
) -> Element {
    let stroke = stroke.unwrap_or(4.0);
    let stroke_color = color.clone().unwrap_or_default().to_string();
    let diameter = radius * 2.0;
    let normalized_radius = radius - stroke * 2.0;
    let circumference = normalized_radius * 2.0 * std::f32::consts::PI;
    let stroke_dash_offset = circumference - percent / 100.0 * circumference;

    // console::log_1(&format!("stroke: {}", stroke).into());
    // console::log_1(&format!("stroke_color: {}", stroke_color).into());
    // console::log_1(&format!("radius: {}", radius).into());
    // console::log_1(&format!("diameter: {}", diameter).into());
    // console::log_1(&format!("normalized_radius: {}", normalized_radius).into());
    // console::log_1(&format!("circumference: {}", circumference).into());
    // console::log_1(&format!("percent: {}", percent).into());
    // console::log_1(&format!("stroke_dash_offset: {}", stroke_dash_offset).into());

    cx.render(rsx! {
        svg {
            width: "{diameter}",
            height: "{diameter}",

            circle {
                class: "{stroke_color} transition-[stroke-dashoffset] duration-[35ms] -rotate-90 origin-center translate-x-0",
                stroke_dasharray: "{circumference} {circumference}",
                stroke_dashoffset: "{stroke_dash_offset}",
                stroke_width: "{stroke}",
                fill: "transparent",
                r: "{normalized_radius}",
                cx: "{radius}",
                cy: "{radius}"
            }
        }
    })
}

fn get_breakdown(current_time: &Date) -> (u32, u32, u32, u32, u32, u32, u32) {
    let year = current_time.get_full_year();
    let month = current_time.get_month();
    let week = 0u32;
    let day = current_time.get_date();
    let hour = current_time.get_hours();
    let minute = current_time.get_minutes();
    let second = current_time.get_seconds();

    (year, month, week, day, hour, minute, second)
}

fn get_percentages(
    breakdown: (u32, u32, u32, u32, u32, u32, u32),
) -> (u32, u32, u32, u32, u32, u32, u32) {
    unimplemented!()
}
