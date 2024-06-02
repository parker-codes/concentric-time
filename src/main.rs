use core::time::Duration;
use js_sys::Date;
use leptos::*;

pub fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (current_time, set_current_time) = create_signal(Date::new_0());
    set_interval(
        move || {
            set_current_time(Date::new_0());
        },
        Duration::from_secs(1),
    );

    let breakdown = move || get_breakdown(&current_time());
    let percentages = move || get_percentages(breakdown());

    // create_effect(move |_| {
    //     logging::log!("minute_percentage = {}", minute_percentage());
    // });

    view! {
        <div class="p-4 w-full h-full flex flex-col justify-center items-center sm:gap-y-12 dark:bg-slate-900">
            <TimeDisplay time={current_time} />

            <div class="grid grid-cols-1 grid-rows-1 place-items-center scale-75 sm:scale-100">
                <Ring label="Minute" percent={move || percentages().0} radius={220.0} color={RingColor::Violet} />
                <Ring label="Hour" percent={move || percentages().1} radius={180.0} color={RingColor::Blue} />
                <Ring label="Day" percent={move || percentages().2} radius={140.0} color={RingColor::Green} />
                <Ring label="Month" percent={move || percentages().3} radius={100.0} color={RingColor::Yellow} />
                <Ring label="Year" percent={move || percentages().4} radius={60.0} color={RingColor::Red} />
            </div>

            <Appropriation />
        </div>
    }
}

#[component]
fn TimeDisplay(time: ReadSignal<Date>) -> impl IntoView {
    let date = move || time().to_date_string().as_string();
    let time = move || time().to_locale_time_string("en-US").as_string();

    view! {
        <div class="flex flex-col items-end gap-y-2 tracking-wide font-bold">
            <span class="text-xl text-gray-500 dark:text-gray-400">
                {date}
            </span>
            <span class="text-4xl sm:text-7xl text-gray-700 dark:text-gray-300">
                {time}
            </span>
        </div>
    }
}

#[component]
fn Ring(
    #[prop(optional, into)] label: String,
    percent: impl Fn() -> f32 + 'static,
    radius: f32,
    #[prop(optional)] color: RingColor,
    #[prop(optional, default = 10.0)] stroke: f32,
    #[prop(optional)] class: String,
) -> impl IntoView {
    let stroke_color = move || color.as_stroke();
    let fill_color = move || color.as_fill();
    let diameter = move || radius * 2.0;
    let normalized_radius = move || radius - stroke * 2.0;
    let circumference = move || normalized_radius() * 2.0 * std::f32::consts::PI;
    let stroke_dash_offset = move || circumference() - percent() / 100.0 * circumference();

    view! {
        <svg
            class="group row-start-1 row-span-1 col-start-1 col-span-1"
            width={diameter}
            height={diameter}
        >
            <circle
                class={format!("transition-[stroke-dashoffset] duration-1000 -rotate-90 origin-center translate-x-0 rounded-[50%] outline-dotted outline-2 outline-transparent group-focus-within:outline-slate-200 {} {}", class, stroke_color())}
                tabindex="0"
                stroke-dasharray={format!("{} {}", circumference(), circumference())}
                stroke-dashoffset={stroke_dash_offset}
                stroke-width={stroke}
                stroke-linecap="round"
                fill="transparent"
                r={normalized_radius}
                cx={radius}
                cy={radius}
            />

            <defs>
                <path
                    id={format!("label-{}", label)}
                    d={format!(r#"
                        M 0,{radius}
                        a {radius},{radius} 0 1,1 {diameter},0
                        {radius},{radius} 0 1,1 -{diameter},0
                    "#, radius = radius, diameter = diameter())}
                />
            </defs>

            <text
                class="origin-center rotate-90 opacity-0 transition-opacity duration-500 group-hover:opacity-100 group-focus-within:opacity-100 tracking-widest"
            >
                <textPath
                    class={format!("text-xs font-bold {} {}", stroke_color(), fill_color())}
                    href={format!("#label-{}", label)}
                    alignment-baseline="hanging"
                >
                    {label}
                </textPath>
            </text>
        </svg>
    }
}

#[derive(PartialEq, Clone, Copy)]
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

#[component]
fn Appropriation() -> impl IntoView {
    view! {
        <div class="group fixed bottom-10 right-0 left-2">
            <div class="transition-opacity group-hover:opacity-0 absolute right-6">
                <Heart />
            </div>

            <div class="transition-opacity opacity-0 group-hover:opacity-100 group-focus-within:opacity-100 absolute right-2 xs:right-6 text-center text-xs text-gray-700 dark:text-gray-400">
                "Made with "
                <Heart />
                " by Parker McMullin (aka. "
                <a href="https://twitter.com/parker_codes" target="_blank" tabindex="0">"@parker_codes"</a>
                ")"
            </div>
        </div>
    }
}

#[component]
fn Heart() -> impl IntoView {
    view! {
        <span class="text-red-500 dark:text-red-700">
            "❤"
        </span>
    }
}

/*
 * Helpers
 */

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
