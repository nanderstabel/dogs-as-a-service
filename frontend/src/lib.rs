use chrono::NaiveDateTime;
use gloo::console::log;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use yew::prelude::*;

#[derive(Clone)]
pub struct State {
    pub dogs: Option<Vec<Dog>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Dog {
    pub id: String,
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub date: NaiveDateTime,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| State { dogs: None });
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:3000/api/v1/dog/all")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<Dog>>()
                    .await
                    .unwrap();

                let mut new_state = state.deref().clone();
                new_state.dogs = Some(response);
                state.set(new_state);
            });
        })
    };
    html! {
        <div>
            <h1>{"App"}</h1>
            <button {onclick}>{"get tasks"}</button>
            <pre>{serde_json::to_string_pretty(&state.dogs).unwrap()}</pre>
        </div>
    }
}
