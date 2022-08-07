use yew::prelude::*;

use crate::front_end::atoms::tile::Tile;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub board: Vec<Vec<char>>
}

#[function_component(TileGrid)]
pub fn tile_grid(props: &Props) -> Html {
    let mut tile_rows = Vec::new();

    for i in 0..6 {
        let mut tile_row = Vec::new();

        for j in 0..7 {
            let color = props.board[i][j];
            let mut class = "blue col s1".to_string();
            if j == 0 { class.push_str(" offset-s2") };

            tile_row.push(html!{
                <div {class}>
                    <Tile {color} /> 
                </div>
            });
        }

        tile_rows.push(html!{
            <div class="row" style="margin-bottom: 0px">
                {tile_row}
            </div>
        });
    }

    html!{<>{tile_rows}</>}
}