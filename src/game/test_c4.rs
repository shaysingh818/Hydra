#[cfg(test)]
mod game_logic {

    /* unit tests for connect 4 environment */
    use crate::agent::Agent;
    use crate::board::Board;
    use crate::c4::*;

    #[test]
    fn test_left_right_diagonal() {
        let mut board: Board = Board::new(7, 6);
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
        board.place_piece(3, 3, agent1);

        let result = check_diagonals(&board, agent1.get_piece());

        board.print_board();
        assert_eq!(result, true);
    }

    #[test]
    fn test_right_left_diagonal() {
        let mut board: Board = Board::new(7, 6);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        board.place_piece(0, 3, agent1);
        board.place_piece(1, 2, agent1);
        board.place_piece(2, 1, agent1);
        board.place_piece(3, 0, agent1);

        let result = check_diagonals(&board, agent1.get_piece());

        board.print_board();
        assert_eq!(result, true);
    }

    #[test]
    fn test_verticals() {
        let mut board: Board = Board::new(7, 6);
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
        board.place_piece(1, 2, agent1);
        board.place_piece(2, 2, agent1);
        board.place_piece(3, 2, agent1);

        let mut result = check_verticals(&board, agent1.get_piece());
        board.print_board();
        println!("result: {:?}", result);
        assert_eq!(result, true);

        /* test offset */
        board.clear();
        board.place_piece(1, 2, agent1);
        board.place_piece(2, 2, agent1);
        board.place_piece(3, 2, agent1);
        board.place_piece(4, 2, agent1);

        result = check_verticals(&board, agent1.get_piece());
        board.print_board();
        println!("result: {:?}", result);
        assert_eq!(result, true);

        /* test offset again */
        board.clear();
        board.place_piece(2, 2, agent1);
        board.place_piece(3, 2, agent1);
        board.place_piece(4, 2, agent1);
        board.place_piece(5, 2, agent1);

        result = check_verticals(&board, agent1.get_piece());
        board.print_board();
        println!("result: {:?}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_all_verticals() {
        let mut board: Board = Board::new(7, 6);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        for _start_index in 0..7 {
            for _start in 0..3 {
                let mut shift_index = _start;
                board.clear();
                for _row in 0..4 {
                    //println!("{:?} : {:?}", shift_index, _start_index);
                    board.place_piece(shift_index, _start_index, agent1);
                    shift_index += 1;
                }
                board.print_board();
                let loop_result = check_verticals(&board, agent1.get_piece());
                assert_eq!(loop_result, true);
            }
        }
    }

    #[test]
    fn test_horizontals() {
        let mut board: Board = Board::new(7, 6);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        let _rows = board.get_rows();
        board.place_piece(0, 0, agent1);
        board.place_piece(0, 1, agent1);
        board.place_piece(0, 2, agent1);
        board.place_piece(0, 3, agent1);

        let mut result = check_horizontals(&board, agent1.get_piece());
        board.print_board();
        println!("result: {:?}", result);
        assert_eq!(result, true);

        // test offset
        board.clear();
        board.place_piece(0, 1, agent1);
        board.place_piece(0, 2, agent1);
        board.place_piece(0, 3, agent1);
        board.place_piece(0, 4, agent1);

        result = check_horizontals(&board, agent1.get_piece());
        board.print_board();
        println!("result: {:?}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_all_horizonals() {
        let mut board: Board = Board::new(7, 6);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        agent1.set_score(0);
        agent2.set_score(0);

        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
        for _start_index in 0..6 {
            for _start in 0..4 {
                let mut shift_index = _start;
                board.clear();
                for _row in 0..4 {
                    println!("{:?} : {:?}", _start_index, shift_index);
                    board.place_piece(_start_index, shift_index, agent1);
                    shift_index += 1;
                }
                board.print_board();
                let loop_result = check_horizontals(&board, agent1.get_piece());
                assert_eq!(loop_result, true);
            }
        }
    }
}
