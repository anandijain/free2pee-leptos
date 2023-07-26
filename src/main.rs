
use free2pee_leptos::fetch_example;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(fetch_example)
}

// #[component]
// pub fn SimpleCounter(
//     cx: Scope,
//     /// The starting value for the counter
//     initial_value: i32,
//     /// The change that should be applied each time the button is clicked.
//     step: i32,
// ) -> impl IntoView {
    // let (value, set_value) = create_signal(cx, initial_value);
    // let (lat, set_lat) = create_signal(cx, 0.0);
    // let (lon, set_lon) = create_signal(cx, 0.0);
    // let w = window().unwrap();
    // let wstr = format!("{:?}", w);

    // let nav = w.navigator();
    // let nstr = format!("{:?}", nav);
    // let geo = nav.geolocation().unwrap();
    // let geostr = format!("{:?}", geo);
//     // log!(count);
//     log!("{:?}", nav);

//     // let closure = Closure::wrap(Box::new(move |pos: Position| {
//     //     let pstr = format!("{:?}", pos);
//     //     let coords = pos.coords();
//     //     log!("{:?}", coords);
//     //     set_lat(coords.latitude());
//     //     set_lon(coords.longitude());
//     //     console::log_1(&JsValue::from_str(&pstr));
//     // }) as Box<dyn FnMut(Position)>);

//     // geo.get_current_position(closure.as_ref().unchecked_ref())
//     //     .unwrap();

//     view! { cx,
//         <div>
//             <button on:click=move |_| set_value(0)>"Clear"</button>
//             <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
//             <span>"Value: " {value} "!"</span>
//             <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
//             <p>"Window: " {wstr}</p>
//             <p>"Navigator: " {nstr}</p>
//             <p>"Geolocation: " {geostr}</p>
//             <p>"Latitude: "  {move || lat()}</p>
//             <p>"Longitude: " {move || lon()}</p>
//         </div>
//     }
// }

// pub fn main() {
//     _ = console_log::init_with_level(log::Level::Debug);
//     console_error_panic_hook::set_once();
//     mount_to_body(|cx| {
//         view! { cx,
//             <SimpleCounter
//                 initial_value=0
//                 step=1
//             />
//         }
//     })
// }

// // fn main() {
// //     let w = window().unwrap();
// //     let wstr = format!("{:?}", w);

// //     let nav = w.navigator();
// //     let nstr = format!("{:?}", nav);
// //     let geo = nav.geolocation().unwrap();
// //     let geostr = format!("{:?}", geo);

// //     let (count, set_count) = create_signal(cx, 0);
// //     set_count(1);

// //     mount_to_body(|cx| view! { cx,  <p>"Hello, world!!!"</p> })
// // }
