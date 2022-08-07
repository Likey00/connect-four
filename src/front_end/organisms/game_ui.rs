use yew::prelude::*;
use std::ops::Deref;

use crate::back_end::{ai::AIGame, heuristic::GameState};
use crate::front_end::molecules::{button_row::ButtonRow, tile_grid::TileGrid};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub handle_message: Callback<String>
}

#[function_component(GameUI)]
pub fn game_ui(props: &Props) -> Html {
    let state = use_state(|| AIGame::new(3));
    let in_play = use_state(|| true);
    
    let button_clicked = 
        if in_play.deref().clone() {
            let cloned_state = state.clone();
            let cloned_in_play = in_play.clone();
            let handle_message = props.handle_message.clone();
            
            Callback::from(move |player_move| {
                let mut ai_game_copy = cloned_state.deref().clone();
                let game = &mut ai_game_copy.game;

                if let Err(e) = game.make_move(player_move) {
                    handle_message.emit(e.to_string());
                    return;
                }

                let (msg, is_in_play) = 
                    get_msg(game.is_red, game.get_state());
                
                if !is_in_play {
                    cloned_in_play.set(false);
                    cloned_state.set(ai_game_copy);
                    handle_message.emit(msg);
                    return;
                }

                ai_game_copy.make_best_move();
                let game = &ai_game_copy.game;
                
                let (msg, is_in_play) =
                    get_msg(game.is_red, game.get_state());

                cloned_in_play.set(is_in_play);
                cloned_state.set(ai_game_copy);
                handle_message.emit(msg);
            })
        } 
        else { Callback::from(move |_| ()) };

    let board = state.deref().clone().game.board;
    let disabled = !in_play.deref();

    html!{
        <>
        <ButtonRow {button_clicked} {disabled} />
        <TileGrid {board} />
        </>
    }
}

fn get_msg(is_red: bool, state: GameState) -> (String, bool) {
    match (is_red, state) {
        (false, GameState::Lost) => ("Red Wins!".to_string(), false),
        (true, GameState::Lost) => ("Yellow Wins!".to_string(), false),
        (_, GameState::Draw) => ("It's a Tie!".to_string(), false),
        _ => ("Connect Four".to_string(), true)
    }
}