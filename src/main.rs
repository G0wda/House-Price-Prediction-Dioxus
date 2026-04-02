#![allow(non_snake_case)]
use dioxus::prelude::*;
use linfa_linear::FittedLinearRegression;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod load_model;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::new()
            .with_menu(None)
                .with_window(
                    WindowBuilder::new()
                        .with_title("My App")
                        .with_inner_size(LogicalSize::new(1200.0, 600.0)) // Width, Height
                        .with_resizable(true)
                      
                        
                )
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}
    }
}

#[component]
pub fn Hero() -> Element {

    // ✅ Load model ONCE (WASM-safe)
    let model = use_signal(|| {
        let bytes = include_bytes!("../model.bin");
        bincode::deserialize::<FittedLinearRegression<f64>>(bytes).unwrap()
    });

    // ✅ Inputs
    let mut longitude = use_signal(|| 0.0);
    let mut latitude = use_signal(|| 0.0);
    let mut housing_median_age = use_signal(|| 0.0);
    let mut total_rooms = use_signal(|| 0.0);
    let mut total_bed_rooms = use_signal(|| 0.0);
    let mut population = use_signal(|| 0.0);
    let mut household = use_signal(|| 0.0);
    let mut median_income = use_signal(|| 0.0);
    let mut ocean_proximity = use_signal(|| String::new());

    // ✅ Output
    let mut  result = use_signal(|| 0.0);

    // ✅ Inference logic
    let inference = move |_| {
        let input = load_model::HouseInput {
            longitude: longitude(),
            latitude: latitude(),
            housing_median_age: housing_median_age(),
            total_rooms: total_rooms(),
            total_bedrooms: total_bed_rooms(),
            population: population(),
            households: household(),
            median_income: median_income(),
            ocean_proximity: ocean_proximity(),
        };

        let  price = load_model::predict_house_price(&model(), input);
        result.set(price);
    };

    rsx! {
        div {
            class: "main-div",

            h1 {
                "Housing Price Prediction Model Using Rust and Dioxus"
            }

            input {
                r#type: "number",
                placeholder: "Longitude",
                value: "{longitude()}",
                oninput: move |e| longitude.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Latitude",
                value: "{latitude()}",
                oninput: move |e| latitude.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Housing Median Age",
                value: "{housing_median_age()}",
                oninput: move |e| housing_median_age.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Total Rooms",
                value: "{total_rooms()}",
                oninput: move |e| total_rooms.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Total Bed Rooms",
                value: "{total_bed_rooms()}",
                oninput: move |e| total_bed_rooms.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Population",
                value: "{population()}",
                oninput: move |e| population.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Households",
                value: "{household()}",
                oninput: move |e| household.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "number",
                placeholder: "Median Income",
                value: "{median_income()}",
                oninput: move |e| median_income.set(e.parsed().unwrap_or(0.0)),
                 required: true
            }

            input {
                r#type: "text",
                placeholder: "Ocean Proximity (e.g. NEAR BAY)",
                value: "{ocean_proximity()}",
                oninput: move |e| ocean_proximity.set(e.value()),
                 required: true
            }

            button {
                onclick: inference,
                "Predict"
            }

            p {
                "Predicted Price: {result()}"
            }
        }
    }
}