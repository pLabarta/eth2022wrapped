use chrono::Datelike;
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
struct Mes {
    name: String,
    txs: f64,
}

// Convert from YYYY-MM-DD to Month Name
pub fn month_name(date: &str) -> String {
    let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let month = date.month();
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
    month_name.to_string()
}

#[function_component(TxsPorMes)]
pub fn txs_por_mes(props: &Props) -> Html {
    let query = format!(
        "select
    date_trunc('month', block_timestamp::date) as date,
    count(tx_hash) as total_transacciones
  from
    ethereum.core.fact_transactions
  where
    from_address = lower('{}')
    and block_timestamp::date >= '2022-01-01'
  group by
    date
  order by date",
        props.address
    );

    let txs_por_mes: UseStateHandle<Vec<Mes>> = use_state(|| vec![]);
    {
        let txs_por_mes = txs_por_mes.clone();
        use_effect_with_deps(
            move |_| {
                let txs_por_mes = txs_por_mes.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // Post the query
                    let query_id = post_query(&query.to_string()).await;

                    // Wait for the result
                    let token = extract_token(&query_id);
                    let query_result = wait_for_result(&token).await;

                    // Extract the txs_por_mes
                    let fetched_txs_por_mes = match query_result {
                        QueryResult::Finished(data) => data.results.map(|results| {
                            let mut fetched_meses = Vec::new();
                            for result in results {
                                let date = result[0].as_str().unwrap();
                                let txs = result[1].as_f64().unwrap();
                                fetched_meses.push(Mes {
                                    name: month_name(date),
                                    txs,
                                });
                            }
                            let empty_meses = create_empty_meses();
                            replace_empty_meses(empty_meses, fetched_meses)
                        }),
                        _ => panic!("AAAA"),
                    };

                    txs_por_mes.set(fetched_txs_por_mes.unwrap());

                    // gloo_console::log!("txs_por_mes: {:?}", &fetched_txs_por_mes);
                });

                || ()
            },
            (),
        );
    }

    let meses = (*txs_por_mes).clone();

    let meses_for_chart = meses.clone();
    use_effect(move || {
        if !meses_for_chart.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&meses_for_chart.clone()).unwrap();
            crate::bindings::show_txspormes_chart(js_value);
        }
    });

    html! {
        <>

            {
                if meses.is_empty() {
                    html! {

                    }
                } else {
                    html! {
                        <section>

                            <p class="datoclave">{format!("{}", mes_mas_activo(&meses))}</p>
                            <p class="dialogo">{"fue tu mes con más actividad"}</p>

                        </section>
                    }
                }

            }

            {
                if meses.is_empty() {
                    html! {

                    }
                } else {
                    html! {

                        <section>
                            <p class="dialogo">{"Pero en"}</p>
                            <p class="datoclave">{format!("{}", mes_menos_activo(&meses))}</p>
                            <p class="dialogo">{"te olvidaste de tu wallet"}</p>
                            <p>{"(está bueno salir de la compu)"}</p>

                        </section>

                    }
                }

            }


            // Show each Mes in a table
            // <table>
            //     <thead>
            //         <tr>
            //             <th>{ "Mes" }</th>
            //             <th>{ "Transacciones" }</th>
            //         </tr>
            //     </thead>
            //     <tbody>
            //         { for meses.iter().map(|mes| html! {
            //             <tr>
            //                 <td>{ &mes.name }</td>
            //                 <td>{ &mes.txs }</td>
            //             </tr>
            //         })}
            //     </tbody>
            // </table>
            <section>
                <p class="dialogo">{"Estas fueron tus transacciones en el año"}</p>
                <div class="grafico" id="chart-txspormes"></div>
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
            txs: 0.0,
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

fn mes_mas_activo(meses: &Vec<Mes>) -> String {
    let mut mes_mas_activo = meses[0].clone();
    for mes in meses {
        if mes.txs > mes_mas_activo.txs {
            mes_mas_activo = mes.clone();
        }
    }
    mes_mas_activo.name
}

fn mes_menos_activo(meses: &Vec<Mes>) -> String {
    let mut mes_menos_activo = meses[0].clone();
    for mes in meses {
        if mes.txs < mes_menos_activo.txs {
            mes_menos_activo = mes.clone();
        }
    }
    mes_menos_activo.name
}
