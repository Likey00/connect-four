use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub msg: String
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html!{
        <div class="row">
            <div class="col s7 offset-s2">
                <h1 class="center">{props.msg.clone()}</h1>
            </div>
        </div>
    }
}