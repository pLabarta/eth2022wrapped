use yew::prelude::*;

use crate::{
    query::{post_query, wait_for_result},
    shroom::*,
};

use crate::sections::txspormes::month_name;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
struct Mes {
    name: String,
    netflow: f64,
    inflow: f64,
    outflow: f64,
}

#[function_component(InOut)]
pub fn inout(props: &Props) -> Html {
    let query = format!(
        "SELECT
   date_trunc('month', block_timestamp::DATE) AS DATE,
   SUM ( 
        CASE
         WHEN to_address = LOWER('{}') THEN amount_usd 
         WHEN from_address = LOWER('{}') THEN amount_usd * -1
    ELSE 0
    END ) AS netflow,
   SUM (
        CASE
            WHEN to_address = LOWER('{}') THEN amount_usd
   ELSE 0
   END
   ) AS inflow,
    SUM (
            CASE WHEN from_address = LOWER('{}') THEN amount_usd * -1
            ELSE 0
            END     
    ) AS outflow
  FROM
    ethereum.core.ez_token_transfers
  WHERE
    block_timestamp::DATE >= '2022-01-01'
   GROUP BY 1
   ORDER BY 1",
        props.address, props.address, props.address, props.address
    );

    let total = use_state(|| vec![]);
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
                    let fetched_inout_por_mes = match query_result {
                        QueryResult::Finished(data) => data.results.map(|results| {
                            let mut fetched_meses = Vec::new();
                            for result in results {
                                let date = result[0].as_str().unwrap();
                                let netflow = result[1].as_f64().unwrap();
                                let inflow = result[2].as_f64().unwrap();
                                let outflow = result[3].as_f64().unwrap();
                                fetched_meses.push(Mes {
                                    name: month_name(date),
                                    netflow,
                                    inflow,
                                    outflow,
                                });
                            }
                            let empty_meses = create_empty_meses();
                            replace_empty_meses(empty_meses, fetched_meses)
                        }),
                        _ => panic!("AAAA"),
                    };

                    total.set(fetched_inout_por_mes.unwrap());
                });

                || ()
            },
            (),
        );
    }

    let meses_for_chart = (*total).clone();

    use_effect(move || {
        if !meses_for_chart.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&meses_for_chart.clone()).unwrap();
            crate::bindings::show_inout_chart(js_value);
        }
    });

    html! {
        <>
            <section>
                <p class="dialog">{"Pero todos queremos saber"}</p>
                <p class="datoclave">{"cuánto"}</p>
                <p class="dialog">{"movimos"}</p>
            </section>

            <section>
                <div class="grafico" id="chart-inout"></div>
            </section>

        </>
    }
}

// Create Vec of meses with 0 txs
fn create_empty_meses() -> Vec<Mes> {
    let mut meses = Vec::new();
    for month in 1..13 {
        let month_name = match month {
            1 => "Enero",
            2 => "Febrero",
            3 => "Marzo",
            4 => "Abril",
            5 => "Mayo",
            6 => "Junio",
            7 => "Julio",
            8 => "Agosto",
            9 => "Septiembre",
            10 => "Octubre",
            11 => "Noviembre",
            12 => "Diciembre",
            _ => "Unknown",
        };
        meses.push(Mes {
            name: month_name.to_string(),
            netflow: 0.0,
            inflow: 0.0,
            outflow: 0.0,
        });
    }
    meses
}

// Replace meses with 0 txs with fetched meses
fn replace_empty_meses(meses: Vec<Mes>, fetched_meses: Vec<Mes>) -> Vec<Mes> {
    let mut meses = meses;
    for fetched_mes in fetched_meses {
        let index = meses
            .iter()
            .position(|mes| mes.name == fetched_mes.name)
            .unwrap();
        meses[index] = fetched_mes;
    }
    meses
}
