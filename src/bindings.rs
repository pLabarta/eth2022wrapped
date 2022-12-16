use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/chart.js")]
extern "C" {
    #[wasm_bindgen(js_name = "show_chart")]
    pub fn show_chart(chart: JsValue);
}

#[wasm_bindgen(module = "/src/charts/txspormes.js")]
extern "C" {
    #[wasm_bindgen(js_name = "show_txspormes_chart")]
    pub fn show_txspormes_chart(data: JsValue);
}

#[wasm_bindgen(module = "/src/charts/topinteractions.js")]
extern "C" {
    #[wasm_bindgen(js_name = "show_topinteractions_chart")]
    pub fn show_topinteractions_chart(data: JsValue);
}

#[wasm_bindgen(module = "/src/charts/inout.js")]
extern "C" {
    #[wasm_bindgen(js_name = "show_inout_chart")]
    pub fn show_inout_chart(data: JsValue);
}
