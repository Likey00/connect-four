use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub idx: usize,
    pub handle_onclick: Option<Callback<usize>>,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let onclick = match &props.handle_onclick{
        Some(handle_onclick) => {
            let handle_onclick = handle_onclick.clone();
            let idx = props.idx;
            Some(Callback::from(move |_: MouseEvent| handle_onclick.emit(idx)))
        },
        None => None,
    };
    
    let mut class = "btn waves-effect waves-light red lighten-2 center-align".to_string();
    if props.handle_onclick.is_none() { class.push_str(" disabled"); }

    html!{
        <button {class} {onclick}>
            <i class="material-icons">{"arrow_downward"}</i>
        </button>
    }
}