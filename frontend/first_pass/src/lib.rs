// LET THE ATTRIBUTION BE KNOWN
// this effort was heavily inspired by the following
// wasm-bindgen examples:
// https://github.com/rustwasm/wasm-bindgen/tree/master/examples/fetch
// https://github.com/rustwasm/wasm-bindgen/tree/master/examples/dom

extern crate futures;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate web_sys;
#[macro_use]
extern crate serde_derive;

use futures::{future, Future};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// A struct to hold some data from the HTTP request
/// for temp/ph info.
#[derive(Debug, Serialize, Deserialize)]
pub struct TankStatus {
    pub tank: Tank,
    pub temp: Temp,
    pub ph: Ph,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tank {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
    pub f: f32,
    pub c: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ph {
    pub val: f32,
    pub mv: f32,
}

#[wasm_bindgen]
pub fn run() -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init("http://localhost:3000", &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|p: Promise| {
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(p)
        })
        .and_then(|json| {
            let w2 = web_sys::window().unwrap();
            let document = w2.document().expect("expected document");
            let body = document.body().expect("document should have a body");

            let status_r: Result<Vec<TankStatus>, _> = json.into_serde();

            let fmt_status = match status_r {
                Ok(statuses) => statuses
                    .iter()
                    .map(|status| {
                        format!(
                            "Tank {} ({}): {}F, pH {} ({} mv)<br>",
                            status.tank.id,
                            status.tank.name,
                            status.temp.f,
                            status.ph.val,
                            status.ph.mv
                        )
                    })
                    .collect(),
                Err(e) => format!("Error: {}", e),
            };

            let dom_elem = document.create_element("p").unwrap();
            dom_elem.set_inner_html(&fmt_status);

            // Manual cast of `val: Element` to `&Node`, to call the
            // `append_child` method.
            AsRef::<web_sys::Node>::as_ref(&body)
                .append_child(dom_elem.as_ref())
                .unwrap();

            // This doesn't actually get used, but we need
            // to send something.
            future::ok(json)
        });

    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}
