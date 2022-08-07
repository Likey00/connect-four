use yew::prelude::*;

use crate::front_end::atoms::button::Button;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub disabled: bool,
    pub button_clicked: Callback<usize>
}

#[function_component(ButtonRow)]
pub fn button_row(props: &Props) -> Html {
    let handle_onclick = {
        let button_clicked = props.button_clicked.clone();
        Callback::from(move |choice| button_clicked.emit(choice))
    };
    
    let mut buttons = Vec::new();
    let disabled = props.disabled;

    for idx in 0..7 {
        let handle_onclick = handle_onclick.clone();

        let mut class = "blue col s1".to_string();
        if idx == 0 { class.push_str(" offset-s2") };

        buttons.push(html!{
            <div {class}>
                <Button {idx} {disabled} {handle_onclick} />
            </div>
        });
    }
    
    html!{
        <div class="row" style="margin-bottom: 0px">
            {buttons}
        </div>
    }
}