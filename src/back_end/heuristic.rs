use crate::back_end::game_utils::Game;
use std::cmp;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum GameState {
    Won,
    Unknown(i32),
    Draw,
    Lost,
}

use GameState::{Won, Unknown, Draw, Lost};

impl GameState {
    fn to_i32(&self) -> i32 {
        match self {
            Won => i32::MAX,
            Lost => i32::MIN,
            Draw => 0,
            Unknown(num) => *num,
        }
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_i32().cmp(&other.to_i32())
    }
}

impl GameState {
    pub fn invert(&self) -> GameState {
        match self {
            Won => Lost,
            Lost => Won,
            Draw => Draw,
            Unknown(score) => Unknown(-score),
        }
    }
}

impl Game {
    pub fn get_state(&self) -> GameState {
        if self.levels.iter().all(|x| *x == 0) { return Draw; }

        let dir_states = vec![
            self.get_state_red_horiz(),
            self.get_state_red_vert(),
            self.get_state_red_diag1_down(),
            self.get_state_red_diag1_right(),
            self.get_state_red_diag2_down(),
            self.get_state_red_diag2_right(),
        ];

        let mut red_score = 0;

        for dir_state in dir_states {
            match dir_state {
                Unknown(update) => red_score += update,
                _ => return if self.is_red { dir_state } else { dir_state.invert() },
            }
        }
        
        let to_return = Unknown(red_score);
        if self.is_red { to_return } else { to_return.invert() }
    }

    fn get_state_red_horiz(&self) -> GameState {
        let mut red_score = 0;
        
        for i in 0..6 {
            let (mut red_count, mut yellow_count) = (0, 0);

            for j in 0..7 {
                if self.board[i][j] == '🔴' { red_count += 1; }
                if self.board[i][j] == '🟡' { yellow_count += 1; }

                if j < 3 { continue; }
                
                let state = Game::local_game_state(i, red_count, yellow_count);
                match state {
                    Unknown(update) => red_score += update,
                    _ => return state,
                }

                if self.board[i][j-3] == '🔴' { red_count -= 1; }
                if self.board[i][j-3] == '🟡' { yellow_count -= 1; }
            }
        }
        
        GameState::Unknown(red_score)
    }

    fn get_state_red_vert(&self) -> GameState {
        let mut red_score = 0;

        for j in 0..7 {
            let (mut red_count, mut yellow_count) = (0, 0);

            for i in 0..6 {
                if self.board[i][j] == '🔴' { red_count += 1; }
                if self.board[i][j] == '🟡' { yellow_count += 1; }

                if i < 3 { continue; }

                let state = Game::local_game_state(i-1, red_count, yellow_count);
                match state {
                    Unknown(update) => red_score += update,
                    _ => return state,
                }

                if self.board[i-3][j] == '🔴' { red_count -= 1; }
                if self.board[i-3][j] == '🟡' { yellow_count -= 1; }
            }
        }

        Unknown(red_score)
    }

    fn get_state_red_diag1_down(&self) -> GameState {
        let mut red_score = 0;

        for i in 0..3 {
            let (mut red_count, mut yellow_count) = (0, 0);

            for offset in 0..6-i {
                if self.board[i+offset][offset] == '🔴' { red_count += 1; }
                if self.board[i+offset][offset] == '🟡' { yellow_count += 1; }
                
                if offset < 3 { continue; }

                let state = Game::local_game_state(i+offset-1, red_count, yellow_count);
                match state {
                    Unknown(update) => red_score += update,
                    _ => return state,
                }

                if self.board[i+offset-3][offset-3] == '🔴' { red_count -= 1; }
                if self.board[i+offset-3][offset-3] == '🟡' { yellow_count -= 1; }
            }
        }

        Unknown(red_score)
    }

    fn get_state_red_diag1_right(&self) -> GameState {
        let mut red_score = 0;
        
        for j in 1..4 {
            let (mut red_count, mut yellow_count) = (0, 0);

            for offset in 0..cmp::min(7-j, 6) {
                if self.board[offset][j+offset] == '🔴' { red_count += 1; }
                if self.board[offset][j+offset] == '🟡' { yellow_count += 1; }
                
                if offset < 3 { continue; }

                let state = Game::local_game_state(offset-1, red_count, yellow_count);
                match state {
                    Unknown(update) => red_score += update,
                    _ => return state,
                }

                if self.board[offset-3][j+offset-3] == '🔴' { red_count -= 1; }
                if self.board[offset-3][j+offset-3] == '🟡' { yellow_count -= 1; }
            }
        }

        GameState::Unknown(red_score)
    }

    fn get_state_red_diag2_down(&self) -> GameState {
        let mut red_score = 0;

        for i in 3..6 {
            let (mut red_count, mut yellow_count) = (0, 0);

            for offset in 0..i+1 {
                if self.board[i-offset][offset] == '🔴' { red_count += 1; }
                if self.board[i-offset][offset] == '🟡' { yellow_count += 1; }
                
                if offset < 3 { continue; }

                let state = Game::local_game_state(i-offset+3-1, red_count, yellow_count);
                match state {
                    Unknown(update) => red_score += update,
                    _ => return state,
                }

                if self.board[i-offset+3][offset-3] == '🔴' { red_count -= 1; }
                if self.board[i-offset+3][offset-3] == '🟡' { yellow_count -= 1; }
            }
        } 

        Unknown(red_score)
    }

    fn get_state_red_diag2_right(&self) -> GameState {
        let mut red_score = 0;
        
        for j in 1..4 {
            let (mut red_count, mut yellow_count) = (0, 0);

            for offset in 0..cmp::min(7-j, 6) {
                if self.board[5-offset][j+offset] == '🔴' { red_count += 1; }
                if self.board[5-offset][j+offset] == '🟡' { yellow_count += 1; }
                
                if offset < 3 { continue; }
                
                let state = Game::local_game_state(5-offset+3-1, red_count, yellow_count);
                match state {
                    Unknown(update) => red_score += update,
                    _ => return state,
                }
                
                if self.board[5-offset+3][j+offset-3] == '🔴' { red_count -= 1; }
                if self.board[5-offset+3][j+offset-3] == '🟡' { yellow_count -= 1; }
            }
        }

        Unknown(red_score)
    }

    fn local_game_state(i: usize, red_count: i32, yellow_count: i32) -> GameState {
        if red_count == 4 { return Won; }
        if yellow_count == 4 { return Lost; }
    
        let mut score_update = 0;
    
        if red_count == 0 || yellow_count == 0 {
            if red_count == 1 { score_update += i as i32 + 1; }
            if red_count == 2 { score_update += 3 * (i as i32 + 1); }
            if red_count == 3 { score_update += 9 * (i as i32 + 1); }
    
            if yellow_count == 1 { score_update -= i as i32 + 1; }
            if yellow_count == 2 { score_update -= 3 * (i as i32 + 1); }
            if yellow_count == 3 { score_update -= 9 * (i as i32 + 1); }
        }
    
        Unknown(score_update)
    }
}