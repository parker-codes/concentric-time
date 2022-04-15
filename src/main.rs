#![allow(non_snake_case)]

use async_timer::Interval;
use core::time::Duration;
use dioxus::{core::to_owned, prelude::*};
use js_sys::Date;
use web_sys::console;

fn main() {
    console::log_1(&"Hello, world!".into());
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    let current_time = use_state(&cx, || Date::new_0());

    let breakdown = get_breakdown(current_time.get());
    let percentages = get_percentages(breakdown);

    use_coroutine(&cx, |_rx: UnboundedReceiver<()>| {
        to_owned![current_time];
        let mut interval = Interval::platform_new(Duration::from_secs(1));
        async move {
            loop {
                interval.wait().await;
                current_time.set(Date::new_0());
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "p-4 w-full h-full flex flex-col justify-center items-center sm:gap-y-12 dark:bg-slate-900",

            TimeDisplay { time: current_time.get().clone() }

            div {
                class: "grid grid-cols-1 grid-rows-1 place-items-center scale-75 sm:scale-100",
                Ring { label: "Minute".into(), percent: percentages.4, radius: 220.0, color: RingColor::Violet }
                Ring { label: "Hour".into(),   percent: percentages.3, radius: 180.0, color: RingColor::Blue }
                Ring { label: "Day".into(),    percent: percentages.2, radius: 140.0, color: RingColor::Green }
                Ring { label: "Month".into(),  percent: percentages.1, radius: 100.0, color: RingColor::Yellow }
                Ring { label: "Year".into(),   percent: percentages.0, radius: 60.0,  color: RingColor::Red }
            }

            Appropriation {}
        }
    })
}

#[derive(PartialEq, Clone)]
enum RingColor {
    Gray,
    Red,
    Yellow,
    Green,
    Blue,
    Violet,
}
impl Default for RingColor {
    fn default() -> Self {
        RingColor::Gray
    }
}
impl RingColor {
    fn as_stroke(&self) -> String {
        match self {
            RingColor::Gray => String::from("stroke-gray-500 dark:stroke-gray-400"),
            RingColor::Red => String::from("stroke-red-500"),
            RingColor::Yellow => String::from("stroke-yellow-400"),
            RingColor::Green => String::from("stroke-green-500"),
            RingColor::Blue => String::from("stroke-blue-500"),
            RingColor::Violet => String::from("stroke-violet-600 dark:stroke-violet-500"),
        }
    }

    fn as_fill(&self) -> String {
        match self {
            RingColor::Gray => String::from("fill-gray-500 dark:fill-gray-400"),
            RingColor::Red => String::from("fill-red-500"),
            RingColor::Yellow => String::from("fill-yellow-400"),
            RingColor::Green => String::from("fill-green-500"),
            RingColor::Blue => String::from("fill-blue-500"),
            RingColor::Violet => String::from("fill-violet-600 dark:fill-violet-500"),
        }
    }
}

#[inline_props]
fn TimeDisplay(cx: Scope, time: Date) -> Element {
    let date = time.to_date_string();
    let time = time.to_locale_time_string("en-US");

    cx.render(rsx! {
        div {
            class: "flex flex-col items-end gap-y-2 tracking-wide font-bold",
            span {
                class: "text-xl text-gray-500 dark:text-gray-400",
                "{date}"
            }
            span {
                class: "text-4xl sm:text-7xl text-gray-700 dark:text-gray-300",
                "{time}"
            }
        }
    })
}

#[inline_props]
fn Ring(
    cx: Scope,
    label: Option<String>,
    percent: f32,
    radius: f32,
    color: Option<RingColor>,
    stroke: Option<f32>,
    class: Option<String>,
) -> Element {
    let label = label.clone().unwrap_or_default();
    let stroke = stroke.unwrap_or(10.0);
    let color = color.clone().unwrap_or_default();
    let stroke_color = color.as_stroke();
    let fill_color = color.as_fill();
    let diameter = radius * 2.0;
    let normalized_radius = radius - stroke * 2.0;
    let outer_path = radius + stroke;
    let circumference = normalized_radius * 2.0 * std::f32::consts::PI;
    let stroke_dash_offset = circumference - percent / 100.0 * circumference;
    let class = class.clone().unwrap_or_default();

    cx.render(rsx! {
        svg {
            class: "group row-start-1 row-span-1 col-start-1 col-span-1",
            width: "{diameter}",
            height: "{diameter}",

            circle {
                class: "{class} {stroke_color} transition-[stroke-dashoffset] duration-1000 -rotate-90 origin-center translate-x-0",
                stroke_dasharray: "{circumference} {circumference}",
                stroke_dashoffset: "{stroke_dash_offset}",
                stroke_width: "{stroke}",
                stroke_linecap: "round",
                fill: "transparent",
                r: "{normalized_radius}",
                cx: "{radius}",
                cy: "{radius}"
            }

            defs {
                circle {
                    id: "label-{label}",
                    stroke_dasharray: "{circumference} {circumference}",
                    r: "{normalized_radius}",
                    cx: "{outer_path}",
                    cy: "{outer_path}"
                }
            }

            text {
                class: "origin-center -rotate-90 -translate-x-1 transition-opacity duration-500 opacity-0 group-hover:opacity-100",
                textPath {
                    href: "#label-{label}",
                    class: "text-xs font-bold {stroke_color} {fill_color}",
                    "{label}"
                }
            }
        }
    })
}

#[inline_props]
fn Appropriation(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "group fixed bottom-10 right-0 left-2",
            div {
                class: "transition-opacity group-hover:opacity-0 absolute right-6",
                Heart {}
            }
            div {
                class: "transition-opacity opacity-0 group-hover:opacity-100 absolute right-2 xs:right-6 text-center text-xs text-gray-700 dark:text-gray-400",
                "Made with "
                Heart {}
                " by Parker McMullin (aka. "
                a {
                    href: "https://twitter.com/parker_codes",
                    target: "_blank",
                    "@parker_codes"
                }
                ")"
            }
        }
    })
}

#[inline_props]
fn Heart(cx: Scope) -> Element {
    cx.render(rsx! {
        span {
            class: "text-red-500 dark:text-red-700",
            "❤"
        }
    })
}

fn get_breakdown(current_time: &Date) -> (u32, u32, u32, u32, u32) {
    let month = current_time.get_month() + 1;
    // TODO: determine day of week
    let day = current_time.get_date();
    let hour = current_time.get_hours();
    let minute = current_time.get_minutes();
    let second = current_time.get_seconds();

    (month, day, hour, minute, second)
}

const SECONDS_IN_MINUTE: f32 = 60.0;
const SECONDS_IN_HOUR: f32 = SECONDS_IN_MINUTE * 60.0;
const SECONDS_IN_DAY: f32 = SECONDS_IN_HOUR * 24.0;
const SECONDS_IN_MONTH: f32 = SECONDS_IN_DAY * 30.42; // TODO: not technically correct - should base on how many days are in current month
const SECONDS_IN_YEAR: f32 = SECONDS_IN_MONTH * 12.0;

fn get_percentages(breakdown: (u32, u32, u32, u32, u32)) -> (f32, f32, f32, f32, f32) {
    // get amounts in seconds
    let seconds = breakdown.4 as f32;
    let minutes = breakdown.3 as f32 * SECONDS_IN_MINUTE;
    let hours = breakdown.2 as f32 * SECONDS_IN_HOUR;
    let days = breakdown.1 as f32 * SECONDS_IN_DAY;
    let months = breakdown.0 as f32 * SECONDS_IN_MONTH;

    let minute_percentage = seconds / SECONDS_IN_MINUTE * 100.0;
    let hour_percentage = (minutes + seconds) / SECONDS_IN_HOUR * 100.0;
    let day_percentage = (hours + minutes + seconds) / SECONDS_IN_DAY * 100.0;
    let month_percentage = (days + hours + minutes + seconds) / SECONDS_IN_MONTH * 100.0;
    let year_percentage = (months + days + hours + minutes + seconds) / SECONDS_IN_YEAR * 100.0;

    (
        year_percentage,
        month_percentage,
        day_percentage,
        hour_percentage,
        minute_percentage,
    )
}
