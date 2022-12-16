pub mod bindings;
pub mod chart;
pub mod fakedata;
pub mod query;
pub mod sections;
pub mod shroom;

use yew::prelude::*;

use crate::sections::{InOut, Netflow, TopInteractions, TotalTxs, TxsPorMes};

#[function_component(App)]
pub fn app() -> Html {
    // Este Address no se puede modificar
    let address: UseStateHandle<String> =
        use_state(|| "0x86d3e894b5cdb6a80afffd35ed348868fb98dd3f".to_owned());

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

            <section>
                <p class="dialogo">{"Tu address de Ethereum es:"}</p>
                <p class="dialogo">{{(*address).clone()}}</p>
            </section>

            <TotalTxs address = {(*address).clone()} />

            <Netflow address = {(*address).clone()} />

            <TxsPorMes address = {(*address).clone()} />

            <TopInteractions address = {(*address).clone()} />

            <InOut address = {(*address).clone()} />

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
