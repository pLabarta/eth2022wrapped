use yew::prelude::*;

use crate::{
    query::{post_query, wait_for_result},
    shroom::*,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub address: String,
}

#[function_component(TotalTxs)]
pub fn total_txs(props: &Props) -> Html {
    let query = format!(
        "select
    count(tx_hash) as total_transacciones
  from
    ethereum.core.fact_transactions
  where
    from_address = lower('{}')
    and block_timestamp::date >= '2022-01-01'",
        props.address
    );

    let total = use_state(|| 0.0f64);
    {
        let total = total.clone();
        use_effect_with_deps(
            move |_| {
                let total = total.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // Post the query
                    let query_id = post_query(&query.to_string()).await;

                    // Wait for the result
                    let token = extract_token(&query_id);
                    let query_result = wait_for_result(&token).await;

                    // Extract the total
                    let fetched_total = match query_result {
                        QueryResult::Finished(data) => {
                            data.results.unwrap()[0][0].as_f64().unwrap()
                        }
                        _ => 0.0,
                    };

                    total.set(fetched_total);

                    gloo_console::log!("Total: {}", &fetched_total.to_string());
                });

                || ()
            },
            (),
        );
    }

    html! {
        <section>
            <p class="dialogo">{"Este año hiciste"}</p>
            <p class="datoclave">{format!("{}", &total.to_string())}</p>
            <p class="dialogo">{"transacciones"}</p>
        </section>
    }
}
