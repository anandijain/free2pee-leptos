use leptos::{error::Result, *};
use thiserror::Error;
use serde_json::Value;

#[derive(Error, Clone, Debug)]
pub enum BathroomError {
    #[error("Failed to fetch bathrooms.")]
    FetchBathroomsFailed,
}

async fn fetch_bathrooms(_: ()) -> Result<Value> {
    let lat = 42.3593101;
    let lon = -71.105846;

    // make the request
    let res = reqwasm::http::Request::get(&format!(
        "https://overpass-api.de/api/interpreter?data=[out:json];node[\"amenity\"=\"toilets\"](around:1000,{lat},{lon});out;",
    ))
    .send()
    .await?
    // convert it to JSON
    .json::<Value>()
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
                view! { cx, <pre>{serde_json::to_string_pretty(&data).unwrap()}</pre> }
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
