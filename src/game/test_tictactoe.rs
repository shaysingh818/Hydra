
#[cfg(test)]
mod game_logic {

    use crate::agent::Agent;
    use crate::board::Board;
    use crate::game::tictactoe::*;

    #[test]
    fn test_left_right_diagonal() {
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        board.place_piece(0, 0, agent1);
        board.place_piece(1, 1, agent1);
        board.place_piece(2, 2, agent1);

        let result = diagonals(&board, agent1);
        assert_eq!(result, true);
    }

    #[test]
    fn test_right_left_diagonal() {
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        board.place_piece(0, 2, agent1);
        board.place_piece(1, 1, agent1);
        board.place_piece(2, 0, agent1);

        let result = diagonals(&board, agent1);
        assert_eq!(result, true);
    }

    #[test]
    fn test_verticals() {
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        board.place_piece(0, 0, agent1);
        board.place_piece(1, 0, agent1);
        board.place_piece(2, 0, agent1);

        let result = vert_horiz(&board, agent1);
        assert_eq!(result, true);
    }

    #[test]
    fn test_horizontals() {
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        board.place_piece(0, 0, agent1);
        board.place_piece(0, 1, agent1);
        board.place_piece(0, 2, agent1);

        let result = vert_horiz(&board, agent1);
        assert_eq!(result, true);
    }

    #[test]
    fn test_higher_dimension_boards() {
        let mut board: Board = Board::new(5, 5);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        agent1.set_status(true);

        board.place_piece(4, 0, agent1);
        board.place_piece(4, 1, agent1);
        board.place_piece(4, 2, agent1);
        board.place_piece(4, 3, agent1);
        board.place_piece(4, 4, agent1);

        let result = vert_horiz(&board, agent1);
        assert_eq!(result, true);
    }
}



#[cfg(test)]
mod wasm_integration {

    use crate::agent::Agent;
    use crate::board::Board;
    use crate::game::tictactoe::*;


    #[test]
    fn test_set_board() {
        

    }

}


