#[cfg(test)]
mod tests {
    use crate::Colour;
    use crate::Game;
    use crate::GameState;
    use crate::Piece;

    //TODO
    //CHECK EVERY IMPORTANT FUNCTION THAT IS REQUIRED
    //MAKE MOVE
    //SET PROMOTION, PROMOTION

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
    /// Tests if make_move function properly moves a piece to a different position
    #[test]
    fn move_works() {
        let mut game = Game::new();
        game.make_move("a2".to_string(), "a3".to_string());
        let pos = game.get_index(&"a3".to_string());
        println!("{:?}", pos);
        //result we want
        let exp_res: Option<(Piece, Colour)> = Some((Piece::Peasant, Colour::White));
        assert_eq!(game.board[pos.1][pos.0], exp_res);
    }
    #[test]
    fn promotion_test() {
        /* let mut game = Game::new();
        game.board = [[None; 8]; 8];
        game.board[6][0] = Some((Piece::Peasant, Colour::White));
        game.make_move("a7".to_string(), "a8".to_string());
        let final_pos = game.what_is_on(&"a8".to_string());
        assert_eq!(Some((Piece::Queen, Colour::White)), final_pos); */
    }
}
