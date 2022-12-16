pub mod bindings;
pub mod chart;
pub mod fakedata;
pub mod query;
pub mod sections;
pub mod shroom;

use std::collections::HashMap;

use chart::*;
use gloo_net::http::Request;
use shroom::*;
use yew::prelude::*;

use crate::sections::{InOut, Netflow, TopInteractions, TotalTxs, TxsPorMes};

// #[tokio::main]
// async fn main() {
//     println!("Sending req");

//     let query_string = "select sum(amount_usd) from ethereum.core.ez_token_transfers
//   where to_address = lower('0x65392485b8d869e59b5b2a3cf7de815ed16939aa')
//   and block_timestamp::date >= '2022-01-01'"
//         .to_string();

//     let query_id = post_query(&query_string).await;
//     let token = extract_token(&query_id);
//     println!("Query ID: {}", token);

//     let query_result = wait_for_result(&token).await;
//     println!("Query Result: {}", query_result);
// }

#[function_component(App)]
pub fn app() -> Html {
    let address = "0x65392485b8d869e59b5b2a3cf7de815ed16939aa".to_string();

    html! {
        <>
            <section>
                <h1>{"Your "}<span class="grande">{"2022"}</span>{" Ethereum Wrapped"}</h1>
                <p>{"Todo lo que pasó por tu wallet en 2022"}</p>
                <p>{"(y lo que no, también)"}</p>

                <div class="team">
                    <img src="static/img/bunny.png" />
                    <p>{"By Anomalous Cibercafé"}</p>
                </div>
                <div class="team">
                    <img src="static/img/think.png" />
                    <p>{"Powered by Think & Dev"}</p>
                </div>
            </section>

            <TotalTxs address = {address.clone()} />

            <Netflow address = {address.clone()} />

            <TxsPorMes address = {address.clone()} />

            <TopInteractions address = {address.clone()} />

            <InOut address = {address.clone()} />

            // <button>{"Draw chart"}</button>
            // <svg id="chart" style="width: 100%; height: 800px;"></svg>
            // <Chart />

            // Cantidad de TXs

            // Cantidad de TXs por mes

            //
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
