mod back_end;
mod front_end;

use yew::prelude::*;
use std::ops::Deref;

use front_end::{atoms::header::Header, organisms::game_ui::GameUI};

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| "Connect Four".to_owned());

    let handle_message = {
        let cloned_state = state.clone();

        Callback::from(move |msg| cloned_state.set(msg))
    };
    
    let msg = state.deref().clone();

    html!{
        <div class="container">
            <Header {msg} />
            <div class="divider"></div>
            <GameUI {handle_message} />
        </div>
    }
}
