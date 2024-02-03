
mod tictactoe {

    use crate::environment::agent::Agent;
    use crate::games::tictactoe::TicTacToe;


    #[test]
    fn test_add_agent() {

        let mut board = TicTacToe::new(vec![3, 3], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1).unwrap();
        board.add_agent(agent2).unwrap();

        let mut counter = 0;     
        let expected_ids = [1, 2];
        for item in board.players() {
            assert_eq!(item.id(), expected_ids[counter]); 
            counter += 1; 
        }
    }

    #[test]
    fn test_diagonals() {
        
        let mut board = TicTacToe::new(vec![3, 3], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();
        
        board.place_piece(agent1.id(), vec![0, 0]); 
        board.place_piece(agent1.id(), vec![1, 1]);
        board.place_piece(agent1.id(), vec![2, 2]);

        let result = board.diagonals(agent1.clone());
        assert_eq!(result, true);

        let mut fail_state = TicTacToe::new(vec![3, 3], 2);
        fail_state.add_agent(agent1.clone()).unwrap();
        fail_state.add_agent(agent2.clone()).unwrap();
        
        fail_state.place_piece(agent1.id(), vec![0, 0]); 
        fail_state.place_piece(agent1.id(), vec![0, 1]);
        fail_state.place_piece(agent1.id(), vec![2, 2]);

        let fail_result = fail_state.diagonals(agent1.clone());
        assert_eq!(fail_result, false);

    }


    #[test]
    fn test_horizontals() {

        let mut board = TicTacToe::new(vec![3, 3], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");
        
        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();
        
        board.place_piece(agent1.id(), vec![0, 0]); 
        board.place_piece(agent1.id(), vec![0, 1]);
        board.place_piece(agent1.id(), vec![0, 2]);

        let result = board.vert_horiz(agent1.clone());
        assert_eq!(result, true);

        let mut board_two = TicTacToe::new(vec![3, 3], 2);
        board_two.add_agent(agent1.clone()).unwrap();
        board_two.add_agent(agent2.clone()).unwrap();

        board_two.place_piece(agent1.id(), vec![1, 0]); 
        board_two.place_piece(agent1.id(), vec![1, 1]);
        board_two.place_piece(agent1.id(), vec![1, 2]);
        
        let result_two = board_two.vert_horiz(agent1.clone());
        assert_eq!(result_two, true);

        let mut board_three = TicTacToe::new(vec![3, 3], 2);
        board_three.add_agent(agent1.clone()).unwrap();
        board_three.add_agent(agent2.clone()).unwrap();
 
        board_three.place_piece(agent1.id(), vec![2, 0]); 
        board_three.place_piece(agent1.id(), vec![2, 1]);
        board_three.place_piece(agent1.id(), vec![2, 2]);

        let result_three = board_three.vert_horiz(agent1.clone());
        assert_eq!(result_three, true);

        let mut fail_state = TicTacToe::new(vec![3, 3], 2);
        fail_state.add_agent(agent1.clone()).unwrap();
        fail_state.add_agent(agent2.clone()).unwrap();
        
        fail_state.place_piece(agent1.id(), vec![0, 0]); 
        fail_state.place_piece(agent1.id(), vec![1, 1]);
        fail_state.place_piece(agent1.id(), vec![2, 2]);

        let fail_result = fail_state.vert_horiz(agent1.clone());
        assert_eq!(fail_result, false);
    }


    #[test]
    fn test_verticals() {
        
        let mut board = TicTacToe::new(vec![3, 3], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();
        
        board.place_piece(agent1.id(), vec![0, 0]); 
        board.place_piece(agent1.id(), vec![1, 0]);
        board.place_piece(agent1.id(), vec![2, 0]);

        let result = board.vert_horiz(agent1.clone());
        assert_eq!(result, true);

        let mut board_two = TicTacToe::new(vec![3, 3], 2);
        board_two.add_agent(agent1.clone()).unwrap();
        board_two.add_agent(agent2.clone()).unwrap();
        
        board_two.place_piece(agent1.id(), vec![0, 1]); 
        board_two.place_piece(agent1.id(), vec![1, 1]);
        board_two.place_piece(agent1.id(), vec![2, 1]);

        let result_two = board_two.vert_horiz(agent1.clone());
        assert_eq!(result_two, true);

        let mut board_three = TicTacToe::new(vec![6, 6], 2);
        board_three.add_agent(agent1.clone()).unwrap();
        board_three.add_agent(agent2.clone()).unwrap();
        
        board_three.place_piece(agent1.id(), vec![0, 4]); 
        board_three.place_piece(agent1.id(), vec![1, 4]);
        board_three.place_piece(agent1.id(), vec![2, 4]);
        board_three.place_piece(agent1.id(), vec![3, 4]);
        board_three.place_piece(agent1.id(), vec![4, 4]);
        board_three.place_piece(agent1.id(), vec![5, 4]);

        let result_three = board_three.vert_horiz(agent1);
        assert_eq!(result_three, true);

    }


    #[test]
    fn test_winner() {
        
        let mut board = TicTacToe::new(vec![3, 3], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();
        
        board.place_piece(agent1.id(), vec![0, 0]); 
        board.place_piece(agent1.id(), vec![1, 0]);
        board.place_piece(agent1.id(), vec![2, 0]);

        let result = board.winner(agent1.clone());
        assert_eq!(result, true);

        let mut board_two = TicTacToe::new(vec![3, 3], 2);
        board_two.add_agent(agent1.clone()).unwrap();
        board_two.add_agent(agent2.clone()).unwrap();
        
        board_two.place_piece(agent2.id(), vec![0, 0]); 
        board_two.place_piece(agent2.id(), vec![1, 1]);
        board_two.place_piece(agent2.id(), vec![2, 2]);

        let result = board_two.winner(agent2.clone());
        assert_eq!(result, true);

        let mut board_three = TicTacToe::new(vec![3, 3], 2);
        board_three.add_agent(agent1.clone()).unwrap();
        board_three.add_agent(agent2.clone()).unwrap();
        
        board_three.place_piece(agent2.id(), vec![0, 0]); 
        board_three.place_piece(agent2.id(), vec![1, 1]);
        let result = board_three.winner(agent2);
        assert_eq!(result, false);

    }


    #[test]
    fn test_static_evaluation() {

        let mut board = TicTacToe::new(vec![3, 3], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();

        board.place_piece(agent1.id(), vec![0, 0]); 
        board.place_piece(agent1.id(), vec![0, 1]);
        board.place_piece(agent2.id(), vec![0, 2]);

        let score = board.static_evaluation(
            agent1.clone(), 
            agent2.clone()
        ).unwrap();

        assert_eq!(score, 0);

        let mut board_two = TicTacToe::new(vec![3, 3], 2);
        board_two.add_agent(agent1.clone()).unwrap();
        board_two.add_agent(agent2.clone()).unwrap();

        board_two.place_piece(agent1.id(), vec![0, 0]); 
        board_two.place_piece(agent2.id(), vec![0, 1]);
        board_two.place_piece(agent1.id(), vec![1, 1]);
        board_two.place_piece(agent2.id(), vec![0, 2]);
        board_two.place_piece(agent1.id(), vec![2, 2]);

        let score = board_two.static_evaluation(
            agent1.clone(), 
            agent2.clone()
        ).unwrap();

        assert_eq!(score, 10);
    }

    #[test]
    fn test_pop_piece() {

        let mut board = TicTacToe::new(vec![3, 3], 2);
        let mut agent1 = Agent::new(1, "agent-1");
        let mut agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();
       
        board.place_piece(agent1.id(), vec![0, 2]); 
        board.place_piece(agent2.id(), vec![0, 1]);
        board.place_piece(agent2.id(), vec![1, 1]);
        board.pop_piece();

    }


}


mod gridworld {

    use crate::environment::agent::Agent;
    use crate::games::gridworld::GridWorld;


    #[test]
    fn test_create_gridworld() {

        let mut board = GridWorld::new(vec![3, 4], 1);
        let mut agent1 = Agent::new(2, "agent-1");
        board.add_agent(agent1.clone()).unwrap();

        /* set start and end state */
        board.set_start_pos(vec![2, 0], agent1.id());
        board.set_end_pos(vec![0, 3], 1); 
        board.set_end_pos(vec![1, 3], -1);
        
        // println!("{:?}", board.end_positions());
        let expected_start_pos = vec![2, 0]; 
        let expected_end_pos: Vec<Vec<usize>> = vec![vec![0, 3], vec![1, 3]];
        let expected_actions = vec![vec![2, 1], vec![1, 0]]; 
        assert_eq!(board.end_positions(), &expected_end_pos);
        assert_eq!(board.start_pos(), &expected_start_pos);
        assert_eq!(board.avaliable_moves().unwrap(), expected_actions); 

        let mut counter = 0;     
        let expected_ids = [2];
        for item in board.agents() {
            assert_eq!(item.id(), expected_ids[counter]); 
            counter += 1; 
        }

        counter = 0; 
        let expected_state = vec![0,0,0,1,0,0,0,-1,2,0,0,0]; 
        for item in board.state().grid().values() {
            assert_eq!(&expected_state[counter], item);
            counter += 1; 
        }    
    }


    #[test]
    fn test_action_space() {

        let mut board = GridWorld::new(vec![3, 4], 1);
        let mut agent1 = Agent::new(2, "agent-1");
        board.add_agent(agent1.clone()).unwrap();

        /* set start and end state */
        board.set_start_pos(vec![2, 0], agent1.id());
        board.set_end_pos(vec![0, 3], 1); 
        board.set_end_pos(vec![1, 3], -1);

        let expected_actions = vec![vec![2, 1], vec![1, 0]]; 
        assert_eq!(board.avaliable_moves().unwrap(), expected_actions);
       
        let expected_vals = vec![0, 0, 0, 1, 0, 0, 0, -1, 0, 2, 0, 0];  
        board.take_action(vec![2, 1], agent1.id());
        assert_eq!(board.state().grid().values(), &expected_vals);

        let expected_vals = vec![0, 0, 0, 1, 2, 0, 0, -1, 0, 0, 0, 0];  
        board.take_action(vec![1, 0], agent1.id()); 
        assert_eq!(board.state().grid().values(), &expected_vals);
 
        let expected_actions = vec![vec![1, 1], vec![0, 0], vec![2, 0]]; 
        assert_eq!(board.avaliable_moves().unwrap(), expected_actions);

        let expected_vals = vec![0, 0, 0, 1, 0, 2, 0, -1, 0, 0, 0, 0];  
        board.take_action(vec![1, 1], agent1.id()); 
        assert_eq!(board.state().grid().values(), &expected_vals);

        let expected_vals = vec![2, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0];  
        board.take_action(vec![0, 0], agent1.id());
        assert_eq!(board.state().grid().values(), &expected_vals);
    }
}



mod connect4 {

    use crate::environment::agent::Agent;
    use crate::games::c4::Connect4;


    #[test]
    fn test_create_c4() {

        let mut board = Connect4::new(vec![6, 7], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();

        let mut counter = 0;     
        let expected_ids = [1, 2];
        for item in board.agents() {
            assert_eq!(item.id(), expected_ids[counter]); 
            counter += 1; 
        }

        let mut board_two = Connect4::new(vec![6, 7], 1);
        board_two.add_agent(agent1.clone()).unwrap();
        match board_two.add_agent(agent2.clone()) {
            Ok(_) => println!("Fail due to max player limit"),
            Err(err) => {
                assert_eq!(err, "Max player limit hit"); 
            }
        }

    }


    #[test]
    fn test_drop_piece() {

        let mut board = Connect4::new(vec![6, 7], 2);
        let agent1 = Agent::new(1, "agent-1");
        let agent2 = Agent::new(2, "agent-2");

        board.add_agent(agent1.clone()).unwrap();
        board.add_agent(agent2.clone()).unwrap();

        
        board.drop_piece(agent1.id(), 0); 
        board.state_view();


    }


}