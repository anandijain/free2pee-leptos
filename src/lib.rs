use leptos::{error::Result, *};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;
use js_sys;
use js_sys::Function;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, Geolocation, Navigator, Position, PositionOptions, PositionError, Window};

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
    // let lat = 42.3593101;
    // let lon = -71.105846;
    let window = window().expect("should have a window in this context");
    let navigator = window.navigator();

    let closure = Closure::wrap(Box::new(move |position: Position| async {
        let latitude = position.coords().latitude();
        let longitude = position.coords().longitude();
        let res = reqwasm::http::Request::get(&format!(
            "https://overpass-api.de/api/interpreter?data=[out:json];node[\"amenity\"=\"toilets\"](around:1000,{latitude},{longitude});out;",
        ))
        .send()
        .await.unwrap()
        .json::<OverpassResponse>()
        .await.unwrap();
    log!("{}", &format!("Latitude: {}, Longitude: {}", latitude, longitude));
        return Ok(res);
    }) as Box<dyn FnMut(_)>);

    let error = Closure::wrap(Box::new(move |_error: PositionError| {
        log!("Error occurred while fetching the position.");
    }) as Box<dyn FnMut(_)>);

    navigator.geolocation().unwrap().get_current_position(
        closure.as_ref().unchecked_ref(),
        // error.as_ref().unchecked_ref(),
        // PositionOptions::new().enable_high_accuracy(true),
    );

    closure.forget();
    error.forget();


    Ok(())

    // BathroomError::FetchBathroomsFailed.into()
}

#[component]
fn SimpleExample(cx: Scope) -> impl IntoView {
  let (name, set_name) = create_signal(cx, "The name is ThePrimeagen".to_string());

  let on_change = move |ev| { // ev is inferred to be `web_sys::MouseEvent`
    set_name(event_target_value(&ev));
  };

  view! { cx,
    <input on:change=on_change type="text" value=name/>
  }
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
                let bathroom_elements = data.elements.iter().map(|element| {
                    view! { cx,
                        <tr>
                            <td>
                                <a href={format!("https://www.openstreetmap.org/node/{}", element.id)} target="_blank">OSM:{element.id}</a>
                            </td>
                            <td>
                                <a href={format!("https://www.google.com/maps/dir/?api=1&destination={},{}", element.lat, element.lon)} target="_blank">"Directions"</a>
                            </td>
                        </tr>
                    }
                }).collect_view(cx);
    
                view! { cx,
                    <table>
                        {bathroom_elements}
                    </table>
                }
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
        {SimpleExample(cx)}
    }
}
