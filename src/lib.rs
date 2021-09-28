use std::{fmt, process::Output};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Peasant,
}

pub struct Game {
    /* save board, active colour, ... */
    board: [[Option<(Piece, Colour)>; 8]; 8],
    active_colour: Colour,
    state: GameState,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Self {
        //Use all types in Piece and Colour
        use Colour::*;
        use Piece::*;
        Self {
            /* initialise board, set active colour to white, ... */
            board: [
                [
                    Some((Bishop, White)),
                    Some((Knight, White)),
                    Some((Rook, White)),
                    Some((Queen, White)),
                    Some((King, White)),
                    Some((Knight, White)),
                    Some((Rook, White)),
                    Some((Bishop, White)),
                ],
                [
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                    Some((Peasant, White)),
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                    Some((Peasant, Black)),
                ],
                [
                    Some((Bishop, Black)),
                    Some((Knight, Black)),
                    Some((Rook, Black)),
                    Some((Queen, Black)),
                    Some((King, Black)),
                    Some((Knight, Black)),
                    Some((Rook, Black)),
                    Some((Bishop, Black)),
                ],
            ],
            active_colour: White,
            state: GameState::InProgress,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        //check if inProgress
        if self.get_game_state() == GameState::InProgress {
            if self.get_possible_moves(&_from).unwrap().contains(&_to) {
                let _from = self.get_index(&_from);
                let _to = self.get_index(&_to);
                //> this is ugly
                self.board[_from.0 as usize][_from.1 as usize] = self.board[_to.0 as usize][_to.1 as usize];
            }
        }
        //Find what piece is on a position DONE
        //find if move is legal DOING
        //move piece
        //check if Gamestate has changed
        //return GameState
        None
    }

    pub fn what_is_on(&self, _tile: &String) -> Option<(Piece, Colour)> {
        //Get index for position and return whatever is on that index
        let tile = self.get_index(_tile);
        self.board[tile.0 as usize][tile.1 as usize]
    }
    pub fn get_index(&self, _tile: &String) -> (usize, usize) {
        //split string and turn into seperate variables
        let row: char = _tile[..1].parse().ok().unwrap();
        let column: usize = _tile[1..].parse().ok().unwrap();
        let row: usize = match row {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            //if it is outside of the scope it's always mapped to 0 CHANGE ASAP
            _ => 0,
        };
        (row, column)
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
    pub fn get_possible_moves(&self, _position: &String) -> Option<Vec<String>> {
        //find out what piece is on the given tile
        //match case for possible legal moves
        let piece = self.what_is_on(&_position);
        use Piece::*;
        match piece.unwrap().0 {
            King => self.king_moves(_position),
            Queen => self.queen_moves(_position),
            Knight => self.knight_moves(_position),
            Rook => self.rook_moves(_position),
            Bishop => self.bishop_moves(_position),
            Peasant => self.peasant_moves(_position),
        }
    }
    ///return a position relative to a given one
    pub fn relative_pos(&self, _pos: &String, _row: i8, _column: i8) -> Option<String>{
        let _pos = self.get_index(&_pos);
        let output: (usize, usize) = ((_pos.0 as i8 +_row) as usize, (_pos.1 as i8 +_column)as usize);
        Some(self.index_to_string(output))
    }
    pub fn index_to_string(&self, _input: (usize, usize)) -> String {
        let mut output: String  = String::with_capacity(2);
        output.push(match _input.0 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => ' ',
        });
        //ERROR IF USIZE IS OUTSIDE OF SCOPE
        output.push(char::from_digit(_input.1 as u32, 10).unwrap());
        output
    }
    /// Get the positions that the king can move to from its current position
    pub fn king_moves(&self, _pos: &String) -> Option<Vec<String>> {
        //Get every position surrounding the piece
        let index = self.get_index(_pos);
        //WITH CAPACITY?
        let mut output: Vec<String> = Vec::with_capacity(8);
        for r in -1..=1 {
            for c in -1..=1 {

                let possible_pos = self.relative_pos(_pos, r, c).unwrap();
                if _pos != &possible_pos{

                    output.push(possible_pos);
                }
            }
        }
        //Get if positions are illegal ie, piece there, unavailable space
        //convert to Vec with String
        Some(output)
    }

    /// Get the positions that the queen can move to from its current position
    pub fn queen_moves(&self, _from: &String) -> Option<Vec<String>> {
        None
    }
    /// Get the positions that a knight can move to from its current position
    pub fn knight_moves(&self, _from: &String) -> Option<Vec<String>> {
        None
    }
    /// Loop through every unoccupied position in cross from the rooks position
    /// Return every position therein
    /// FIX ASAP
    pub fn rook_moves(&self, _pos: &String) -> Option<Vec<String>> {
        let index = self.get_index(_pos);
        let output: Vec<String> = Vec::new();
        //right
        for n in 1..(8-index.0){
            let up = self.what_is_on(&self.relative_pos(_pos, n, 0).unwrap());
            if &up == None{
                output.push(up);
            }
            else{
                output.push(up);
                break;
            }
        }
        //down
        for n in 0..(8-index.1){
            let down = self.what_is_on(self.relative_pos(_pos, n, 0));
            if &up == None{
                output.push(up);
            }
            else{
                output.push(up);
                break;
            }
        }
        //left
        for n in (1..index.1){
            let left = self.what_is_on(self.relative_pos(_pos, n, 0));
            if &up == None{
                output.push(up);
            }
            else{
                output.push(up);
                break;
            }
        }
        //right
        for n in (1..index.1){
            let up = self.what_is_on(self.relative_pos(_pos, n, 0));
            if &up == None{
                output.push(up);
            }
            else{
                output.push(up);
                break;
            }
        }
        Some(output)
    }
    /// Get the positions that a bishop can move to from its current position
    pub fn bishop_moves(&self, _from: &String) -> Option<Vec<String>> {
        let index = self.get_index(_pos);
        let output: Vec<String> = Vec::new();
        //Up right
        for n in 1..(8-index.0){
            let up = self.what_is_on(&self.relative_pos(n, -n, 0).unwrap());
            if &up == None{
                output.push(up);
            }
            else{
                output.push(up);
                break;
            }
        }
        //down right
        for n in 0..(8-index.1){
            let down = self.what_is_on(&self.relative_pos(n, n, 0).unwrap());
            if &down == None{
                output.push(down);
            }
            else{
                output.push(down);
                break;
            }
        }
        //down left
        for n in (1..index.1){
            let left = self.what_is_on(&self.relative_pos(-n, n, 0).unwrap());
            if &left == None{
                output.push(left);
            }
            else{
                output.push(left);
                break;
            }
        }
        //up left
        for n in (1..index.1){
            let upleft = self.what_is_on(&self.relative_pos(-n, -n, 0).unwrap());
            if &upleft == None{
                output.push(upleft);
            }
            else{
                output.push(upleft);
                break;
            };
        }
        None
    }
    /// Get the positions that a peasant can move to from its current position
    pub fn peasant_moves(&self, _from: &String) -> Option<Vec<String>> {
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
