use std::{collections::HashSet, fmt, process::Output};
mod tests;

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
    promotion: Piece,
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
            promotion: Queen,
            state: GameState::InProgress,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        //check if inProgress
        use Colour::*;
        let colour = self.active_colour;
        let piece = self.what_is_on(&_from).unwrap().0;
        if self.get_game_state() == GameState::InProgress {
            if self.get_possible_moves(&_from).unwrap().contains(&_to) {
                let _from = self.get_index(&_from);
                let _to = self.get_index(&_to);
                //this is ugly
                self.board[_from.0 as usize][_from.1 as usize] =
                    self.board[_to.0 as usize][_to.1 as usize];
                //PROMOTIONS
                if piece == Piece::Peasant {
                    //Check if the peasant is on the opposite side of board
                    //promote the peasants
                    match colour{
                        White => {
                            if _to.0 == 0{
                                self.promote(self.index_to_string(_to));
                            }
                        },
                        Black => {
                            if _to.0 == 7{
                                self.promote(self.index_to_string(_to));
                            }
                        }
                    }
                }
            }
        }
        let king_pos = self.find_king(colour);
        self.king_in_danger(&king_pos, colour);
        //check if Gamestate has changed
        //return GameState
        None
    }
    ///Find king of a certain colour by looping through the board
    fn find_king(&self, colour: Colour) -> String{
        let mut king_pos: String = String::new();
        use Piece::*;
        //find the king
        for rank in 1..=8 {
            for file in 1..=8 {
                if self.board[rank][file].unwrap() == (King, colour) {
                    king_pos = self.index_to_string((rank, file));
                }
            }
        }
        king_pos
    }

    ///Loop through every value that could threaten the square that the king is on
    ///if king is checked, return that DONE
    ///if king is check mated, return that
    ///if not, return in progress
    fn king_in_danger(&self, _pos: &String, colour:Colour) -> Option<GameState> {
        use GameState::*;
        let mut output: Option<GameState>= Some(InProgress);
        let king_moves: Vec<String> =self.king_moves(&_pos).unwrap();
        let mut hs: HashSet<String> = HashSet::new();
        for rank in 1..=8{
            for file in 1..=8{
                //if other colour choose piece
                let piece = self.what_is_on(&self.index_to_string((rank,file))).unwrap();
                let piece_pos = self.index_to_string((rank,file));
                let mut pos_moves: Vec<String> = self.get_possible_moves(&piece_pos).unwrap();
                //add all dangerous or occupied position into a hashset
                if piece.1 != colour {

                
                    if pos_moves.contains(&_pos){
                        output = Some(Check);
                        //push piece position into possible move and insert into hashset
                        //THIS IS GONNA CAUSE PROBLEMS MAYBE
                        hs.insert(pos_moves.into_iter().collect());
                        hs.insert(piece_pos);
                    }
                }
                //if the piece is of the same colour include only the square it occupies
                else {
                    hs.insert(piece_pos);
                }
            }
        }
        //compare hashset to the moves the king can make and its current positions. If it includes all these positions
        if  king_moves.iter().all(|king_move| hs.contains(king_move)) && hs.contains(_pos){
            output = Some(GameOver);
        }

        output
    }

    fn what_is_on(&self, _tile: &String) -> Option<(Piece, Colour)> {
        //Get index for position and return whatever is on that index
        let tile = self.get_index(_tile);
        self.board[tile.0 as usize][tile.1 as usize]
    }
    fn get_index(&self, _tile: &String) -> (usize, usize) {
        //split string and turn into seperate variables
        let rank: char = _tile[..1].parse().ok().unwrap();
        let file: usize = _tile[1..].parse().ok().unwrap();
        let rank: usize = match rank {
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
        (rank, file)
    }

    /// Set the piece type that a peasant becames following a promotion.
    /// 
    /// Returns a Some(Piece, Colour)
    pub fn set_promotion(&mut self, _piece: String) -> () {
        use Piece::*;
        match _piece.as_ref(){
            "queen" => self.promotion = Queen,
            "bishop" => self.promotion = Bishop,
            "knight" => self.promotion = Knight,
            "rook" => self.promotion = Rook,
            _ => self.promotion = self.promotion

        };
        ()
    }
    fn promote(&mut self, _pos: String){
        let index = self.get_index(&_pos);
        self.board[index.0][index.1] = Some((self.promotion, self.active_colour));
        
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
            Bishop => self.bishop_moves(_position),
            Knight => self.knight_moves(_position),
            Rook => self.rook_moves(_position),
            Peasant => self.peasant_moves(_position),
        }
    }
    ///return a position relative to a given one
     fn relative_pos(&self, _pos: &String, _rank: i8, _file: i8) -> Option<String> {
        let _pos = self.get_index(&_pos);
        let output: (usize, usize) = (
            (_pos.0 as i8 + _rank) as usize,
            (_pos.1 as i8 + _file) as usize,
        );
        Some(self.index_to_string(output))
    }

    fn index_to_string(&self, _input: (usize, usize)) -> String {
        let mut output: String = String::with_capacity(2);
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
    fn king_moves(&self, _pos: &String) -> Option<Vec<String>> {
        //Get every position in an 3x3 grid centered on the position
        //Remove occupied positions and the starting position
        //check if the new position puts the king in check
        //convert to Vec with String
        let mut output: Vec<String> = Vec::with_capacity(8);
        let colour: Colour = self.what_is_on(&_pos).unwrap().1;
        for r in -1..=1 {
            for c in -1..=1 {
                let possible_pos = self.relative_pos(_pos, r, c).unwrap();
                if _pos != &possible_pos && self.what_is_on(&possible_pos) == None && self.king_in_danger(&possible_pos, colour).unwrap() == GameState::InProgress{
                    output.push(possible_pos);
                }
            }
        }
        Some(output)
    }

    /// Get the positions that the queen can move to from its current position
    fn queen_moves(&self, _pos: &String) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        output.append(&mut self.bishop_moves(_pos).unwrap());
        output.append(&mut self.rook_moves(_pos).unwrap());
        Some(output)
    }
    /// Get the positions that a bishop can move to from its current position
    fn bishop_moves(&self, _pos: &String) -> Option<Vec<String>> {
        let index = self.get_index(_pos);
        let mut output: Vec<String> = Vec::new();
        //
        //Up right
        for n in 1..(8 - index.0) {
            let new_pos = self.relative_pos(_pos, -(n as i8), n as i8).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }

        //down right
        for n in 1..(8 - index.1) {
            let new_pos = self.relative_pos(_pos, n as i8, n as i8).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }
        //down left
        for n in 1..index.1 {
            let new_pos = self.relative_pos(_pos, n as i8, -(n as i8)).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }
        //up left
        for n in 1..index.1 {
            let new_pos = self.relative_pos(_pos, -(n as i8), -(n as i8)).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }
        Some(output)
    }

    /// Get the positions that a knight can move to from its current position
    fn knight_moves(&self, _pos: &String) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        //all possible moves relative to position
        let _pot_rank: [i8; 8] = [1, 1, -1, -1, 2, 2, -2, -2];
        let _pot_col: [i8; 8] = [2, -2, 2, -2, 1, -1, 1, -1];
        for n in 0..8 {
            //find position after move
            let pot_pos = self.relative_pos(_pos, _pot_rank[n], _pot_col[n]).unwrap();
            //save position if tile is empty or
            if self.what_is_on(&pot_pos) == None
                || self.what_is_on(&pot_pos).unwrap().1 != self.what_is_on(&_pos).unwrap().1
            {
                output.push(pot_pos);
            }
        }
        Some(output)
    }
    /// Loop through every unoccupied position in cross from the rooks position
    /// Return every position therein
    /// FIX ASAP
    fn rook_moves(&self, _pos: &String) -> Option<Vec<String>> {
        let index = self.get_index(_pos);
        let mut output: Vec<String> = Vec::new();

        //right
        for n in 1..(8 - index.0) {
            let new_pos = self.relative_pos(_pos, 0, n as i8).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }

        //down
        for n in 1..(8 - index.1) {
            let new_pos = self.relative_pos(_pos, n as i8, 0).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }
        //left
        for n in 1..index.1 {
            let new_pos = self.relative_pos(_pos, 0, -(n as i8)).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }
        //up
        for n in 1..index.1 {
            let new_pos = self.relative_pos(_pos, -(n as i8), 0).unwrap();
            if self.what_is_on(&new_pos) == None {
                output.push(new_pos);
            } else {
                output.push(new_pos);
                break;
            }
        }
        Some(output)
    }
    /// Get the positions that a peasant can move to from its current position
    /// If the peasant is in a starting position, include 2 square moves
    fn peasant_moves(&self, _pos: &String) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        match self.active_colour {
            Colour::White => {
                //basic movement
                let new_pos = self.relative_pos(_pos, -1, 0).unwrap();
                if self.what_is_on(&new_pos) == None {
                    output.push(new_pos);
                }

                //if the peasant is diagonally opposed to a piece, allow attack
                let att_pos1 = self.relative_pos(_pos, -1, -1).unwrap();
                if self.what_is_on(&att_pos1).unwrap().1 == Colour::Black {
                    output.push(att_pos1);
                }

                let att_pos2 = self.relative_pos(_pos, -1, 1).unwrap();
                if self.what_is_on(&att_pos2).unwrap().1 == Colour::Black {
                    output.push(att_pos2);
                }
                //double movement if on starting square
                let pot_pos = self.relative_pos(_pos, -2, 0).unwrap();
                if self.get_index(_pos).0 == 7 && self.what_is_on(&pot_pos) == None {
                    output.push(pot_pos);
                }
                //PROMOTIONS!
            }

            Colour::Black => {
                let new_pos = self.relative_pos(_pos, 1, 0).unwrap();
                //basic movement
                if self.what_is_on(&new_pos) == None {
                    output.push(new_pos);
                }

                //if the peasant is diagonally opposed to a piece, allow attack
                let att_pos1 = self.relative_pos(_pos, 1, -1).unwrap();
                if self.what_is_on(&att_pos1).unwrap().1 == Colour::Black {
                    output.push(att_pos1);
                }
                //other attack position
                let att_pos2 = self.relative_pos(_pos, 1, 1).unwrap();
                if self.what_is_on(&att_pos2).unwrap().1 == Colour::Black {
                    output.push(att_pos2);
                }

                //double movement if on starting square
                let pot_pos = self.relative_pos(_pos, 2, 0).unwrap();
                if self.get_index(_pos).0 == 7 && self.what_is_on(&pot_pos) == None {
                    output.push(pot_pos);
                }
            }
        };

        Some(output)
    }
    fn symbol(&self, input: (Piece, Colour)) -> String {
        use Piece::*;
        use Colour::*;
        match input{
            (King, White) => format!("{}", "♚"),
            (King, Black) => format!("{}", "♔"),
            (Queen, White) => format!("{}", "♛"),
            (Queen, Black) => format!("{}", "♕"),
            (Bishop, White) => format!("{}", "♝"),
            (Bishop, Black) => format!("{}", "♗"),
            (Knight, White) => format!("{}", "♞"),
            (Knight, Black) => format!("{}", "♘"),
            (Rook, White) => format!("{}", "♜"),
            (Rook, Black) => format!("{}", "♖"),
            (Peasant, White) => format!("{}", "♟"),
            (Peasant, Black) => format!("{}", "♙"),
            _ => format!("{}", "*")
        }
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
        let mut output: String = String::new();
        let board = self.board;
/*          output.push_str("|:----------------------:|");
         output.push_str(" A B C D E F G H \n");
        for rank in 0..7 {
            for file in 0..7{
                output.push_str("|");
                if file == 7{
                    output.push_str(&rank.to_string());
                    output.push_str("\n");
                }
                print!("{}", &self.symbol(board[2][2].unwrap()));
                //output  += &self.symbol(board[2][2].unwrap());
                if rank == 7 {
                    output.push_str("")
                }
            }
            output.push_str("|:----------------------:|");
        }  */
        
        write!(f, "\n{}", output)
    }
}
