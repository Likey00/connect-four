#[derive(Clone)]
pub struct Game {
    pub board: Vec<Vec<char>>,
    pub levels: Vec<usize>,
    pub is_red: bool,
}

impl Game {
    pub fn new() -> Game {
        Game { board: vec![vec!['⚫'; 7]; 6], levels: vec![6; 7], is_red: true }
    }
    
    pub fn make_move(&mut self, col: usize) -> Result<(), &str> {
        if col >= 7 { return Err("Column index out of range"); }
        if self.levels[col] == 0 { return Err("Column is full"); }

        self.levels[col] -= 1;
        self.board[self.levels[col]][col] = if self.is_red {'🔴'} else {'🟡'};
        self.is_red = !self.is_red;
        Ok(())
    }
    
    pub fn undo_move(&mut self, col: usize) -> Result<(), &str> {
        if col >= 7 { return Err("Column index out of range"); }
        if self.levels[col] == 6 { return Err("Column is empty"); }
        
        self.board[self.levels[col]][col] = '⚫';
        self.levels[col] += 1;
        self.is_red = !self.is_red;
        Ok(())
    }
}