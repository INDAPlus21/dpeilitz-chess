use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour{
    White, Black
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece{
    King, Queen, Rook, Knight, Bishop, Peasant 
}

pub struct Game {
    /* save board, active colour, ... */
    board: [[Option<(Piece, Colour)>; 8]; 8],
    active_colour: Colour,
    state: GameState
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Self {
        //Use all types in Piece and Colour
        use Piece::*;
        use Colour::*;
        Self {
            /* initialise board, set active colour to white, ... */
            board: [
                [Some((Bishop, White)), Some((Knight, White)), Some((Rook, White)), Some((Queen, White)), Some((King, White)), Some((Knight, White)), Some((Rook, White)), Some((Bishop, White))],
                [Some((Peasant, White)), Some((Peasant, White)), Some((Peasant, White)), Some((Peasant, White)), Some((Peasant, White)), Some((Peasant, White)), Some((Peasant, White)), Some((Peasant, White))],
                [None; 8],
                [None; 8],
                [None; 8], 
                [None; 8],
                [Some((Peasant, Black)), Some((Peasant, Black)), Some((Peasant, Black)), Some((Peasant, Black)), Some((Peasant, Black)), Some((Peasant, Black)), Some((Peasant, Black)), Some((Peasant, Black))],
                [Some((Bishop, Black)), Some((Knight, Black)), Some((Rook, Black)), Some((Queen, Black)), Some((King, Black)), Some((Knight, Black)), Some((Rook, Black)), Some((Bishop, Black))]],
            active_colour: White,
            state: GameState::InProgress

        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        //check if inProgress
        if self.get_game_state() == GameState::InProgress{

        }
        //Find what piece is on tile
        //find if move is legal
        //move piece
        //check if Gamestate has changed
        //return GameState
        None
    }
    pub fn what_is_on(&self, _tile: String) -> Option<(Piece, Colour)> {
        let row: &char = &_tile[..1].parse().ok().unwrap();
        let column: &usize = &_tile[1..].parse().ok().unwrap();
        let row = match row {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            //if it is outside of the scope it's always mapped to 0
            _ => 0
            };
            //search position by index and check the occupance
        self.get_tile(&row, &column)

    }
    pub fn get_tile(& self, &row: &usize, &column: &usize) -> Option<(Piece, Colour)>{
        self.board[row][column]
    }
    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        //find the piece that is to be upgraded by looping through the top row
        //Allow the user to input a piece
        //Create match statement for all pieces
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        //find out what piece is on the given tile
        //match case for possible legal moves
        None
    }

    //
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}