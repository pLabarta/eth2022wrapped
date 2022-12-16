use yew::prelude::*;

use crate::{
    query::{post_query, wait_for_result},
    shroom::*,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub address: String,
}

#[function_component(Netflow)]
pub fn netflow(props: &Props) -> Html {
    let query = format!(
        "SELECT
   SUM ( 
        CASE
         WHEN to_address = LOWER('{}') THEN amount_usd 
         WHEN from_address = LOWER('{}') THEN amount_usd * -1
    END ) AS netflow
  FROM
    ethereum.core.ez_token_transfers
  WHERE
    block_timestamp::DATE >= '2022-01-01'",
        props.address, props.address
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

    let total_to_show = (*total).clone();

    html! {
        <section>
            {
                if total_to_show > 0.0 {
                    html! {
                        <>
                            <p class="dialogo">{"¡Estás terminando el año con"}</p>
                            <p class="datoclave">{format!("{:.2}", &total_to_show)}</p>
                            <p class="dialogo">{"más dólarucos en tu cuenta!"}</p>
                        </>
                    }
                } else {
                    html! {
                        <>
                            <p class="dialogo">{"Estás terminando el año"}</p>
                            <p class="datoclave">{format!("{:.2}", &total_to_show)}</p>
                            <p class="dialogo">{"dólares abajo."}</p>
                            <p class="dialogo">{"Lo sentimos."}</p>
                        </>
                    }
                }
            }

        </section>
    }
}
