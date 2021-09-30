#[cfg(test)]
mod tests {
    use crate::Game;
    use crate::GameState;
    //TODO 
    //CHECK EVERY IMPORTANT FUNCTION THAT IS REQUIRED
    //MAKE MOVE
    //SET PROMOTION, PROMOTION

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