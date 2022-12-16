use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::{
    bindings,
    fakedata::{Provider, Tick},
};

#[function_component(Chart)]
pub fn draw_chart() -> Html {
    let chart_data = use_state(|| None);
    {
        let chart_data = chart_data.clone();
        use_effect_with_deps(
            move |_| {
                let chart_data = chart_data.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let provider = Provider {};
                    let data = provider.fetch_chart().await.unwrap();
                    chart_data.set(Some(data));
                })
            },
            (),
        )
    }

    if chart_data.is_none() {
        html! {
            <>
                {"Loading chart..."}
            </>
        }
    } else {
        bindings::show_chart(JsValue::from_serde(&(*chart_data).clone()).unwrap());
        html! {
            <>
            </>
        }
    }
}
