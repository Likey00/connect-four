use crate::back_end::game_utils::Game;
use crate::back_end::heuristic::GameState;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct AIGame {
    pub game: Game,
    pub depth: usize,
}

impl AIGame {
    pub fn new(depth: usize) -> AIGame {
        AIGame { game: Game::new(), depth }
    }

    pub fn make_best_move(&mut self) {
        let (mut best_outcome, mut best_move) = (GameState::Lost, 0);

        for poss_move in 0..=6 {
            if let Some(gs) = self.eval_move(poss_move, self.depth) {
                match gs.cmp(&best_outcome) {
                    Ordering::Less => (),
                    _ => {
                        best_outcome = gs;
                        best_move = poss_move;
                    },
                }
                self.game.undo_move(poss_move).unwrap();
            }
        }

        self.game.make_move(best_move).unwrap();
    }

    fn eval_move(&mut self, test_move: usize, depth: usize) -> Option<GameState> {
        if self.game.make_move(test_move).is_err() {
            return None;
        }
        
        let state = self.game.get_state().invert();
        if depth == 0 || !matches!(state, GameState::Unknown(_)) {
            return Some(state);
        }

        let mut best_opp_outcome = GameState::Lost;

        for opp_move in 0..=6 {
            if let Some(gs) = self.eval_move(opp_move, depth-1) {
                best_opp_outcome = best_opp_outcome.max(gs);
                self.game.undo_move(opp_move).unwrap();
            }
        }

        Some(best_opp_outcome.invert())
    }
}
