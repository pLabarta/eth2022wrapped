use yew::prelude::*;

use crate::{
    query::{post_query, wait_for_result},
    shroom::*,
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
struct Interaction {
    protocol: String,
    txs: f64,
}

#[function_component(TopInteractions)]
pub fn top_interactions(props: &Props) -> Html {
    let query = format!(
        "WITH top_interactions AS (
SELECT to_address, COUNT(*) AS numero_de_transacciones FROM ethereum.core.fact_transactions
   WHERE block_timestamp::DATE >= '2022-01-01'
    AND from_address = LOWER('{}')
    GROUP BY 1
    ORDER BY 2 DESC
),
address_names AS (
  SELECT * FROM ethereum.core.dim_labels 
  WHERE address IN
  (
    SELECT t.to_address FROM top_interactions t 
  )
),
protocols AS (
SELECT * FROM address_names
  FULL OUTER JOIN top_interactions ON address_names.address = top_interactions.to_address
  ORDER BY top_interactions.numero_de_transacciones DESC
)
SELECT COALESCE(label, 'protocolos no identificados') AS protocolo, COUNT(*) AS transacciones FROM protocols 
  GROUP BY 1 
    ORDER BY 2 DESC",
        props.address
    );

    let interactions: UseStateHandle<Vec<Interaction>> = use_state(|| vec![]);
    {
        let interactions = interactions.clone();
        use_effect_with_deps(
            move |_| {
                let interactions = interactions.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // Post the query
                    let query_id = post_query(&query.to_string()).await;
                    gloo_console::log!("top interactions query post: {:?}", &query_id);

                    // Wait for the result
                    let token = extract_token(&query_id);
                    let query_result = wait_for_result(&token).await;

                    // Extract the interactions
                    let fetched_interactions = match query_result {
                        QueryResult::Finished(data) => data.results.map(|rows| {
                            rows.iter()
                                .map(|row| Interaction {
                                    protocol: row[0].as_str().unwrap().to_string(),
                                    txs: row[1].as_f64().unwrap(),
                                })
                                .collect::<Vec<Interaction>>()
                        }),
                        _ => panic!("AAAA"),
                    };

                    interactions.set(fetched_interactions.unwrap());

                    // gloo_console::log!("interactions: {}", &fetched_interactions.to_string());
                });

                || ()
            },
            (),
        );
    }

    let interactions_for_chart = (*interactions).clone();
    let interactions_for_dialog = (*interactions).clone();
    use_effect(move || {
        if !interactions_for_chart.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&interactions_for_chart.clone()).unwrap();
            crate::bindings::show_topinteractions_chart(js_value);
        }
    });

    html! {
        <>


            { if interactions_for_dialog.is_empty() {
                html! {
                    <>
                    </>
                }
            } else {
                html! {
                    <>
                        <section>
                            <p class="dialogo">{"Interactuaste con"}</p>
                            <p class="datoclave">{format!("{}", &interactions_for_dialog.len())}</p>
                            <p class="dialogo">{"protocolos"}</p>
                        </section>

                        <section>
                            <p class="dialogo">{"Tu protocolo favorito fue"}</p>
                            <p class="datoclave">{format!("{}", find_interaction_with_most_txs(&interactions_for_dialog).unwrap().protocol)}</p>
                        </section>
                    </>
                }
            }}



            // <h1>{"Top Interactions"}</h1>
            // <table>
            //     <thead>
            //         <tr>
            //             <th>{"Protocol"}</th>
            //             <th>{"Txs"}</th>
            //         </tr>
            //     </thead>
            //     <tbody>
            //         {for interactions.iter().map(|interaction| html! {
            //             <tr>
            //                 <td>{&interaction.protocol}</td>
            //                 <td>{&interaction.txs.to_string()}</td>
            //             </tr>
            //         })}
            //     </tbody>
            // </table>
            <section>
                <p class="dialogo">{"Tu top #3"}</p>
                <div class="grafico" id="chart-topinteractions"></div>
            </section>

        </>
    }
}

fn find_interaction_with_most_txs(interactions: &Vec<Interaction>) -> Option<&Interaction> {
    interactions
        .iter()
        .max_by(|a, b| a.txs.partial_cmp(&b.txs).unwrap())
}
