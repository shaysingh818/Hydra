

#[cfg(test)]
mod minimax {

    use crate::agent::Agent;
    use crate::board::Board;
    use crate::engine::minimax::*;

    #[test]
    fn test_minimax() {
        /*

        : For this set of tests, the logic for creating board
          configurations can be more modular by looping through
          a set of existing stored configurations.

        */

        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        agent1.set_status(false);
        agent2.set_status(true);

        /* test first board configuration */
        board.place_piece(0, 2, agent1);
        board.place_piece(0, 1, agent2);
        board.place_piece(1, 1, agent2);

        /* call minimax function */
        let (_best_score, _best_move) =
            Board::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

        let mut optimal_move = (2, 1);
        assert_eq!(_best_move, optimal_move);

        /* test second configuration */
        board.clear();
        board.place_piece(0, 2, agent1);
        board.place_piece(1, 0, agent2);
        board.place_piece(1, 2, agent2);

        /* call minimax with second configuration */
        let (_best_score1, _best_move1) =
            Board::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

        optimal_move = (1, 1);
        assert_eq!(_best_move1, optimal_move);

        /* test third configuration (see if agent1 tries to max) */
        board.clear();
        board.place_piece(1, 1, agent1);
        board.place_piece(0, 1, agent2);
        board.place_piece(1, 0, agent2);

        /* call minimax with second configuration */
        let (_best_score2, _best_move2) =
            Board::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

        optimal_move = (0, 0);
        assert_eq!(_best_move2, optimal_move);
    }


    #[test]
    fn test_negamax() {

        /* set up environment */ 
        let mut board: Board = Board::new(3, 3);
        for item in 1..3 {
            let mut temp_agent : Agent = Agent::new(item); 
            temp_agent.set_score(0); 
            board.add_agent(temp_agent); 
        }

        /* create board configuration */
        board.make_move((1,1));
        board.make_move((0,2));
        board.make_move((1,2));

        /* test negamax function */ 
        let mut optimal_move = (1, 0); 
        let (_best_score, _best_move) =
            Board::negamax(&mut board.clone(), 0, 9);

        assert_eq!(_best_move, optimal_move);

        /* testing second situation because I am skeptical */
        board.clear(); 
        board.make_move((0,2));
        board.make_move((0,0));
        board.make_move((1,1));
        board.print_board(); 

        /* test negamax again */ 
        optimal_move = (2, 0); 
        let (_best_score1, _best_move1) =
            Board::negamax(&mut board.clone(), 0, 9);

        assert_eq!(_best_move1, optimal_move);
    }

    #[test]
    fn test_ab_negamax() {

        /* set up environment */ 
        let mut board: Board = Board::new(3, 3);
        for item in 1..3 {
            let mut temp_agent : Agent = Agent::new(item); 
            temp_agent.set_score(0); 
            board.add_agent(temp_agent); 
        }

        /* create board configuration */
        board.make_move((1,1));
        board.make_move((0,2));
        board.make_move((1,2));

        /* test negamax function */ 
        let mut optimal_move = (1, 0); 
        let (_best_score, _best_move) = Board::ab_negamax(
            &mut board.clone(), 0, 9, -1000, 1000
        );

        assert_eq!(_best_move, optimal_move);

        /* testing second board configuration */
        board.clear(); 
        board.make_move((0,2));
        board.make_move((0,0));
        board.make_move((1,1));
        board.print_board(); 

        /* test ab negamax function */ 
        optimal_move = (2, 0); 
        let (_best_score1, _best_move1) = Board::ab_negamax(
            &mut board.clone(), 0, 9, -1000, 1000
        );

        assert_eq!(_best_move1, optimal_move); 
    }

}



#[cfg(test)]
mod game_logic {

    use crate::agent::Agent;
    use crate::board::Board;
    use crate::tictactoe::*;

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
mod board_evaluation {

    use crate::agent::Agent;
    use crate::board::Board;
    use crate::engine::minimax::*;

    #[test]
    fn test_static_evaluation() {

        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        /* place board configuration state */ 
        board.place_piece(0, 0, agent1);
        board.place_piece(0, 1, agent1);
        board.place_piece(0, 2, agent2);

        /* generate score */ 
        let mut score = Board::static_evaluation(&mut board, agent1, agent2); 
        assert_eq!(score, 0);

        /* create board config with winner score */ 
        board.clear(); 
        board.place_piece(0, 0, agent1);
        board.place_piece(0, 1, agent2);
        board.place_piece(1, 1, agent1);
        board.place_piece(0, 2, agent2); 
        board.place_piece(2, 2, agent1);

        score = Board::static_evaluation(&mut board, agent1, agent2);  
        assert_eq!(score, 10);
    } 


    #[test]
    fn test_negmax_eval() {
        
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        /* create board configuration with make move */ 
        board.make_move((0,0));
        board.make_move((0,1));
        board.make_move((1,1));
        board.make_move((0,2));
        board.make_move((2,2));
        board.make_move((2,0));

        let score = Board::negmax_eval(&mut board); 
        assert_eq!(score, 10);
    } 
}

