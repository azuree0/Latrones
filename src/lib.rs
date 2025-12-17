use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub enum Player {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Empty,
    Piece(Player),
}

impl Default for Square {
    fn default() -> Self {
        Square::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GamePhase {
    Placement,
    Movement,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct GameState {
    board: [Square; 64],
    current_player: Player,
    phase: GamePhase,
    light_pieces_placed: u8,
    dark_pieces_placed: u8,
    selected_square: Option<usize>,
    must_continue_jumping: bool, // If true, only jump moves are allowed
    piece_that_captured: Option<usize>, // Track which piece has captured this turn
    game_over: bool,
    winner: Option<Player>,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            board: [Square::Empty; 64],
            current_player: Player::Light,
            phase: GamePhase::Placement,
            light_pieces_placed: 0,
            dark_pieces_placed: 0,
            selected_square: None,
            must_continue_jumping: false,
            piece_that_captured: None,
            game_over: false,
            winner: None,
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn current_player(&self) -> Player {
        self.current_player
    }
    
    #[wasm_bindgen(getter)]
    pub fn game_over(&self) -> bool {
        self.game_over
    }
    
    #[wasm_bindgen(getter)]
    pub fn winner(&self) -> Option<Player> {
        self.winner
    }
    
    #[wasm_bindgen(getter)]
    pub fn phase(&self) -> String {
        match self.phase {
            GamePhase::Placement => "placement".to_string(),
            GamePhase::Movement => "movement".to_string(),
        }
    }
    
    #[wasm_bindgen(getter)]
    pub fn selected_square(&self) -> Option<usize> {
        self.selected_square
    }
    
    pub fn get_board(&self) -> JsValue {
        let board_array: Vec<u8> = self.board.iter().map(|sq| {
            match sq {
                Square::Empty => 0,
                Square::Piece(Player::Light) => 1,
                Square::Piece(Player::Dark) => 2,
            }
        }).collect();
        serde_wasm_bindgen::to_value(&board_array).unwrap()
    }
    
    pub fn get_valid_moves(&self) -> JsValue {
        let mut moves = Vec::new();
        
        match self.phase {
            GamePhase::Placement => {
                // In placement phase, all empty squares are valid
                for i in 0..64 {
                    if matches!(self.board[i], Square::Empty) {
                        moves.push(i);
                    }
                }
            }
            GamePhase::Movement => {
                if let Some(selected) = self.selected_square {
                    // Verify the selected square still has a piece (it might have been captured)
                    if matches!(self.board[selected], Square::Piece(p) if p == self.current_player) {
                        // Get valid destinations from selected square
                        // Use must_continue_jumping flag to filter moves if we're in a jump sequence
                        moves = self.get_valid_destinations(selected);
                    } else {
                        // Selected square is empty or has wrong piece - clear selection
                        // This can happen if the piece was captured
                        // Don't clear here, let select_square handle it
                        moves = Vec::new();
                    }
                } else {
                    // Get all squares that can be selected (player's pieces)
                    // When checking if a piece can be selected, always check ALL moves (not filtered by must_continue_jumping)
                    for i in 0..64 {
                        if let Square::Piece(p) = self.board[i] {
                            if p == self.current_player {
                                // Check if this piece has any valid moves (check all moves, not just jumps)
                                // Use jumps_only=false to see all possible moves when selecting a new piece
                                if !self.get_valid_destinations_internal(i, false).is_empty() {
                                    moves.push(i);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        serde_wasm_bindgen::to_value(&moves).unwrap()
    }
    
    fn get_valid_destinations(&self, from: usize) -> Vec<usize> {
        self.get_valid_destinations_internal(from, self.must_continue_jumping)
    }
    
    fn get_valid_destinations_jumps_only(&self, from: usize) -> Vec<usize> {
        self.get_valid_destinations_internal(from, true)
    }
    
    fn get_valid_destinations_internal(&self, from: usize, jumps_only: bool) -> Vec<usize> {
        let mut destinations = Vec::new();
        
        if from >= 64 {
            return destinations;
        }
        
        let row = from / 8;
        let col = from % 8;
        
        // Check orthogonal directions (up, down, left, right)
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;
            
            if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                let to = (new_row * 8 + new_col) as usize;
                
                match self.board[to] {
                    Square::Empty => {
                        if !jumps_only {
                            destinations.push(to);
                        }
                    }
                    Square::Piece(p) => {
                        // Check if we can jump over this piece
                        if p != self.current_player {
                            let jump_row = new_row + dr;
                            let jump_col = new_col + dc;
                            
                            if jump_row >= 0 && jump_row < 8 && jump_col >= 0 && jump_col < 8 {
                                let jump_to = (jump_row * 8 + jump_col) as usize;
                                if matches!(self.board[jump_to], Square::Empty) {
                                    destinations.push(jump_to);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        destinations
    }
    
    pub fn select_square(&mut self, square: usize) -> bool {
        if square >= 64 || self.game_over {
            return false;
        }
        
        match self.phase {
            GamePhase::Placement => {
                // In placement phase, clicking an empty square places a piece
                if matches!(self.board[square], Square::Empty) {
                    self.board[square] = Square::Piece(self.current_player);
                    
                    match self.current_player {
                        Player::Light => {
                            self.light_pieces_placed += 1;
                            if self.light_pieces_placed == 8 {
                                // Check if all pieces are placed
                                if self.dark_pieces_placed == 8 {
                                    self.phase = GamePhase::Movement;
                                } else {
                                    self.switch_player();
                                }
                            } else {
                                self.switch_player();
                            }
                        }
                        Player::Dark => {
                            self.dark_pieces_placed += 1;
                            if self.dark_pieces_placed == 8 {
                                // All pieces placed, switch to movement phase
                                self.phase = GamePhase::Movement;
                            } else {
                                self.switch_player();
                            }
                        }
                    }
                    
                    self.check_captures();
                    self.check_win_condition();
                    return true;
                }
            }
            GamePhase::Movement => {
                if let Some(selected) = self.selected_square {
                    // Verify the selected square still has a piece (it might have been captured)
                    if !matches!(self.board[selected], Square::Piece(p) if p == self.current_player) {
                        // Selected square is empty or has wrong piece - clear selection
                        self.selected_square = None;
                        self.must_continue_jumping = false;
                        // Fall through to try selecting the clicked square
                    } else {
                        // Check if user clicked on another piece of the same player (reselect)
                        if let Square::Piece(p) = self.board[square] {
                            if p == self.current_player && square != selected {
                                // User clicked on a different piece of the same player - reselect it
                                // When selecting a NEW piece, check ALL moves (not filtered by must_continue_jumping)
                                let valid_destinations = self.get_valid_destinations_internal(square, false);
                                if !valid_destinations.is_empty() {
                                    self.selected_square = Some(square);
                                    self.must_continue_jumping = false; // Reset when selecting a new piece
                                    self.piece_that_captured = None; // Reset capture tracking when selecting new piece
                                    return true;
                                }
                            }
                        }
                        
                        // If we must continue jumping, only allow jump moves
                        if self.must_continue_jumping {
                            let row_from = selected / 8;
                            let col_from = selected % 8;
                            let row_to = square / 8;
                            let col_to = square % 8;
                            let dr = (row_to as i32) - (row_from as i32);
                            let dc = (col_to as i32) - (col_from as i32);
                            let is_jump = dr.abs() == 2 || dc.abs() == 2;
                            
                            if !is_jump {
                                // Trying to make a regular move when jumps are required
                                // But allow reselecting if clicking on another piece
                                if let Square::Piece(p) = self.board[square] {
                                    if p == self.current_player && square != selected {
                                        // User clicked on a different piece - allow reselection
                                        let valid_destinations = self.get_valid_destinations_internal(square, false);
                                        if !valid_destinations.is_empty() {
                                            self.selected_square = Some(square);
                                            self.must_continue_jumping = false;
                                            self.piece_that_captured = None;
                                            return true;
                                        }
                                    }
                                }
                                return false;
                            }
                        }
                        
                        // Try to move to this square
                        if self.make_move(selected, square) {
                            // Check if the piece still exists at the destination (it might have been captured)
                            let piece_still_exists = matches!(self.board[square], Square::Piece(p) if p == self.current_player);
                            
                            // Check if this was a jump and if more jumps are possible
                            let row_from = selected / 8;
                            let col_from = selected % 8;
                            let row_to = square / 8;
                            let col_to = square % 8;
                            let dr = (row_to as i32) - (row_from as i32);
                            let dc = (col_to as i32) - (col_from as i32);
                            
                            let was_jump = dr.abs() == 2 || dc.abs() == 2;
                            
                        if was_jump && piece_still_exists {
                            // Check if more jumps are available from the new position
                            // Only check for jump moves (distance == 2)
                            // But only if this piece hasn't already captured
                            if self.piece_that_captured == Some(square) {
                                // This piece has already captured, no more moves allowed
                                self.selected_square = None;
                                self.must_continue_jumping = false;
                                self.piece_that_captured = None;
                                self.switch_player();
                            } else {
                                let additional_jumps = self.get_valid_destinations_jumps_only(square);
                                
                                if !additional_jumps.is_empty() {
                                    // More jumps available - keep piece selected and only allow jump moves
                                    self.selected_square = Some(square);
                                    self.must_continue_jumping = true;
                                } else {
                                    // No more jumps, deselect and switch player
                                    self.selected_square = None;
                                    self.must_continue_jumping = false;
                                    self.piece_that_captured = None;
                                    self.switch_player();
                                }
                            }
                        } else {
                            // Regular move, or piece was captured - deselect and switch player
                            self.selected_square = None;
                            self.must_continue_jumping = false;
                            self.piece_that_captured = None;
                            self.switch_player();
                        }
                            return true;
                        } else {
                            // Invalid move - keep piece selected so user can try a different destination
                            // Don't deselect here, only deselect when user selects a different piece
                            return false;
                        }
                    }
                }
                
                // No piece selected, or selected piece was invalid - try to select the clicked square
                // Select a piece
                if let Square::Piece(p) = self.board[square] {
                    if p == self.current_player {
                        // When selecting a NEW piece, check ALL moves (not filtered by must_continue_jumping)
                        // The must_continue_jumping flag should only apply when a piece is already selected
                        let valid_destinations = self.get_valid_destinations_internal(square, false);
                        if !valid_destinations.is_empty() {
                            self.selected_square = Some(square);
                            self.must_continue_jumping = false; // Reset when selecting a new piece
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }
    
    fn make_move(&mut self, from: usize, to: usize) -> bool {
        if from >= 64 || to >= 64 {
            return false;
        }
        
        // Check if move is valid - use jumps_only=false to check ALL moves
        // The must_continue_jumping flag is only for UI filtering, not move validation
        let valid_destinations = self.get_valid_destinations_internal(from, false);
        if !valid_destinations.contains(&to) {
            return false;
        }
        
        // Check if this is a jump
        let row_from = from / 8;
        let col_from = from % 8;
        let row_to = to / 8;
        let col_to = to % 8;
        
        let dr = (row_to as i32) - (row_from as i32);
        let dc = (col_to as i32) - (col_from as i32);
        
        // Check if this piece has already captured this turn
        if let Some(captured_piece_pos) = self.piece_that_captured {
            if captured_piece_pos == from {
                // This piece has already captured, don't allow another move
                return false;
            }
        }
        
        // Check if we jumped over an enemy piece
        let mut captured_by_jump = false;
        if dr.abs() == 2 || dc.abs() == 2 {
            // This is a jump
            let jumped_row = row_from + (dr / 2) as usize;
            let jumped_col = col_from + (dc / 2) as usize;
            let jumped_index = jumped_row * 8 + jumped_col;
            
            // Check if there's an enemy piece to jump over
            if let Square::Piece(jumped_piece) = self.board[jumped_index] {
                if jumped_piece != self.current_player {
                    // Remove the jumped piece (capture by jump)
                    self.board[jumped_index] = Square::Empty;
                    captured_by_jump = true;
                }
            }
        }
        
        // Move the piece
        let piece = self.board[from];
        self.board[from] = Square::Empty;
        self.board[to] = piece;
        
        // If this piece captured by jump, mark it and return early
        // Don't check for surrounding captures if it already captured by jump
        if captured_by_jump {
            self.piece_that_captured = Some(to);
            self.check_win_condition();
            return true;
        }
        
        // Check for captures after move (surrounding captures)
        // Note: check_captures might remove the piece we just moved if it gets captured
        let capturing_pieces = self.check_captures();
        
        // If this piece (at 'to') caused a capture by surrounding, mark it
        if capturing_pieces.contains(&to) && self.piece_that_captured.is_none() {
            self.piece_that_captured = Some(to);
        }
        
        // If the piece we just moved was captured, we need to handle that
        // This can happen if moving to a square causes the piece to be surrounded
        if matches!(self.board[to], Square::Empty) {
            // The piece was captured immediately after moving
            // This is valid - the move succeeded, but the piece was captured
            // The selected_square will be handled in select_square
        }
        
        // Note: Player switching is handled in select_square based on whether more jumps are available
        // This allows the UI to keep the piece selected for additional jumps
        
        self.check_win_condition();
        true
    }
    
    fn check_captures(&mut self) -> Vec<usize> {
        // Check all squares for captures
        // Returns list of positions of pieces that caused captures (for the current player)
        let mut captured_squares = Vec::new();
        let mut capturing_pieces = Vec::new(); // Track which pieces caused captures
        
        for i in 0..64 {
            if let Square::Piece(piece_player) = self.board[i] {
                // Check if this piece is surrounded on opposite sides
                let row = i / 8;
                let col = i % 8;
                
                // Check horizontal (left-right)
                let left = if col > 0 { Some(i - 1) } else { None };
                let right = if col < 7 { Some(i + 1) } else { None };
                
                if let (Some(left_idx), Some(right_idx)) = (left, right) {
                    if let (Square::Piece(left_p), Square::Piece(right_p)) = 
                        (self.board[left_idx], self.board[right_idx]) {
                        if left_p != piece_player && right_p != piece_player && left_p == right_p {
                            // Captured horizontally
                            captured_squares.push(i);
                            // Track which pieces caused this capture (only current player's pieces)
                            if left_p == self.current_player {
                                capturing_pieces.push(left_idx);
                            }
                            if right_p == self.current_player {
                                capturing_pieces.push(right_idx);
                            }
                            continue;
                        }
                    }
                }
                
                // Check vertical (up-down)
                let up = if row > 0 { Some(i - 8) } else { None };
                let down = if row < 7 { Some(i + 8) } else { None };
                
                if let (Some(up_idx), Some(down_idx)) = (up, down) {
                    if let (Square::Piece(up_p), Square::Piece(down_p)) = 
                        (self.board[up_idx], self.board[down_idx]) {
                        if up_p != piece_player && down_p != piece_player && up_p == down_p {
                            // Captured vertically
                            captured_squares.push(i);
                            // Track which pieces caused this capture (only current player's pieces)
                            if up_p == self.current_player {
                                capturing_pieces.push(up_idx);
                            }
                            if down_p == self.current_player {
                                capturing_pieces.push(down_idx);
                            }
                        }
                    }
                }
            }
        }
        
        // Apply captures after checking all squares to avoid double-capturing
        for square in captured_squares {
            self.board[square] = Square::Empty;
        }
        
        // Return list of pieces that caused captures
        capturing_pieces
    }
    
    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::Light => Player::Dark,
            Player::Dark => Player::Light,
        };
        // Reset capture tracking when switching players
        self.piece_that_captured = None;
    }
    
    fn check_win_condition(&mut self) {
        if self.phase != GamePhase::Movement {
            return;
        }
        
        // Count pieces
        let light_count = self.board.iter()
            .filter(|s| matches!(s, Square::Piece(Player::Light)))
            .count();
        let dark_count = self.board.iter()
            .filter(|s| matches!(s, Square::Piece(Player::Dark)))
            .count();
        
        // Check if a player has no pieces - the player with pieces left wins
        if light_count == 0 && dark_count > 0 {
            // No Light pieces left, Dark pieces remain - Dark wins
            self.game_over = true;
            self.winner = Some(Player::Dark);
            return;
        }
        if dark_count == 0 && light_count > 0 {
            // No Dark pieces left, Light pieces remain - Light wins
            self.game_over = true;
            self.winner = Some(Player::Light);
            return;
        }
        
        // Check if current player has any valid moves
        let has_valid_moves = (0..64).any(|i| {
            if let Square::Piece(p) = self.board[i] {
                if p == self.current_player {
                    return !self.get_valid_destinations(i).is_empty();
                }
            }
            false
        });
        
        if !has_valid_moves {
            self.game_over = true;
            // The player who cannot move loses
            self.winner = Some(match self.current_player {
                Player::Light => Player::Dark,
                Player::Dark => Player::Light,
            });
        }
    }
    
    pub fn reset(&mut self) {
        *self = GameState::new();
    }
    
    pub fn set_starting_pieces(&mut self) {
        // Clear the board first
        self.board = [Square::Empty; 64];
        self.light_pieces_placed = 0;
        self.dark_pieces_placed = 0;
        self.phase = GamePhase::Placement;
        self.current_player = Player::Light;
        self.selected_square = None;
        self.must_continue_jumping = false;
        self.piece_that_captured = None;
        self.game_over = false;
        self.winner = None;
        
        // Place light pieces on H1-H8 (column 7, rows 0-7)
        for row in 0..8 {
            let index = row * 8 + 7; // Column 7 (H)
            self.board[index] = Square::Piece(Player::Light);
            self.light_pieces_placed += 1;
        }
        
        // Place dark pieces on A1-A8 (column 0, rows 0-7)
        for row in 0..8 {
            let index = row * 8 + 0; // Column 0 (A)
            self.board[index] = Square::Piece(Player::Dark);
            self.dark_pieces_placed += 1;
        }
        
        // All pieces are placed, switch to movement phase
        self.phase = GamePhase::Movement;
        self.current_player = Player::Light;
        
        // Check for any initial captures
        self.check_captures();
        self.check_win_condition();
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

