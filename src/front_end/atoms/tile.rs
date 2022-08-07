use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub color: char,
}

#[function_component(Tile)]
pub fn tile(props: &Props) -> Html {
    html!{
        <p style="font-size: 2.75em; margin: 0px">{props.color}</p>
    }
}