use leptos::{error::Result, *};
use serde_json::Value;
use thiserror::Error;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverpassResponse {
    pub elements: Vec<Element>,
    pub generator: String,
    pub osm3s: Osm3s,
    pub version: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Osm3s {
    pub copyright: String,
    #[serde(rename = "timestamp_osm_base")]
    pub timestamp_osm_base: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub tags: HashMap<String, String>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Error, Clone, Debug)]
pub enum BathroomError {
    #[error("Failed to fetch bathrooms.")]
    FetchBathroomsFailed,
}

async fn fetch_bathrooms(_: ()) -> Result<OverpassResponse> {
    let lat = 42.3593101;
    let lon = -71.105846;

    // make the request
    let res = reqwasm::http::Request::get(&format!(
        "https://overpass-api.de/api/interpreter?data=[out:json];node[\"amenity\"=\"toilets\"](around:1000,{lat},{lon});out;",
    ))
    .send()
    .await?
    // convert it to JSON
    .json::<OverpassResponse>()
    .await?;
    Ok(res)
}

pub fn fetch_example(cx: Scope) -> impl IntoView {
    let bathrooms = create_local_resource(cx, || {}, fetch_bathrooms);

    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li> })
                    .collect_view(cx)
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let bathrooms_view = move || {
        bathrooms.read(cx).map(|data| {
            data.map(|data| {
                data.elements.iter().map(|e| {
                    view! { cx, <h1>{e.id.to_string()}</h1> }
                }).collect_view(cx)
                // let els = &data.clone()["elements"];
                // els.as_array()
                //     .iter()
                //     .map(|e| view! { cx, <h1> {e} </h1> })
                //     .collect_view(cx)
                // for el in els.as_array().unwrap() {
                //     view! { cx, <div>{el["tags"]["name"].as_str().unwrap()}</div> };
                // }
                // view! { cx, <pre>{serde_json::to_string_pretty(&data).unwrap()}</pre> }
                // view! { cx, <h1>{"hi"}</h1> }
            })
        })
    };

    view! { cx,
        <div>
            <h2>"Nearest Bathrooms"</h2>
            <ErrorBoundary fallback>
                <Transition fallback=move || {
                    view! { cx, <div>"Loading (Suspense Fallback)..."</div> }
                }>
                <div>
                    {bathrooms_view}
                </div>
                </Transition>
            </ErrorBoundary>
        </div>
    }
}
