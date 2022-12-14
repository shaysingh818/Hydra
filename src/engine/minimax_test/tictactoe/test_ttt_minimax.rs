

#[cfg(test)]
mod minimax {

    use crate::agent::Agent;
    use crate::board::Board;
    use crate::engine::minimax::*;
    use crate::engine::minimax_test::tictactoe::*;


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
            maximizer::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

        let mut optimal_move = (2, 1);
        assert_eq!(_best_move, optimal_move);

        /* test second configuration */
        board.clear();
        board.place_piece(0, 2, agent1);
        board.place_piece(1, 0, agent2);
        board.place_piece(1, 2, agent2);

        /* call minimax with second configuration */
        let (_best_score1, _best_move1) =
            maximizer::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

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
            maximizer::negamax(&mut board.clone(), 0, 9);

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
            maximizer::negamax(&mut board.clone(), 0, 9);

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
        let (_best_score, _best_move) = maximizer::ab_negamax(
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
        let (_best_score1, _best_move1) = maximizer::ab_negamax(
            &mut board.clone(), 0, 9, -1000, 1000
        );

        assert_eq!(_best_move1, optimal_move); 
    }

}