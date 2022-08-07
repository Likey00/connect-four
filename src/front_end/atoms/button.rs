use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub idx: usize,
    pub disabled: bool,
    pub handle_onclick: Callback<usize>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = {
        let handle_onclick = props.handle_onclick.clone();
        let idx = props.idx;
        Callback::from(move |_: MouseEvent| handle_onclick.emit(idx))
    };
    
    let mut class = "btn waves-effect waves-light red lighten-2 center-align".to_string();
    if props.disabled { class.push_str(" disabled"); }

    html!{
        <button {class} {onclick}>
            <i class="material-icons">{"arrow_downward"}</i>
        </button>
    }
}