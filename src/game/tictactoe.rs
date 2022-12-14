use rand::*;
use std::io;
use std::cmp; 
use std::{thread, time};

use crate::agent::Agent;
use crate::board::Board;
use crate::engine::minimax::tictactoe::*;


pub fn diagonals(board: &Board, agent: Agent) -> bool {
    let mut left_right = true;
    let mut right_left = true;
    let board_matrix = board.get_board();
    let board_size = board.get_board().len();
    let mut row_count = (board.get_rows() - 1) as i32;
    let mut col_counter = 0;

    for _col in 0..board_size {
        if board_matrix[col_counter][row_count as usize] != agent.get_piece() {
            left_right = false;
        }

        if board_matrix[col_counter][col_counter] != agent.get_piece() {
            right_left = false;
        }

        col_counter += 1;
        row_count -= 1;
    }

    if left_right == true || right_left == true {
        return true;
    }

    false
}

pub fn vert_horiz(board: &Board, agent: Agent) -> bool {
    let mut horiz = false;
    let mut vert = false;
    let mut row_index = 0;
    let mut col_index = 0;
    let board_matrix = board.get_board();

    for row in board_matrix {
        let mut temp_horiz = true;
        let mut _temp_vert = true;
        for col in row {
            if *col != agent.get_piece() {
                temp_horiz = false;
            }
            if board_matrix[col_index][row_index] != agent.get_piece() {
                _temp_vert = false;
            }
            col_index += 1;
        }

        if temp_horiz == true || _temp_vert == true {
            if temp_horiz {
                horiz = true;
            }

            if _temp_vert {
                vert = true;
            }
            break;
        }

        row_index += 1;
        col_index = 0;
    }

    if horiz == true || vert == true {
        return true;
    }
    false
}


pub fn take_random_action(board: &mut Board, agent: Agent) {
    let position_vec = board.available_moves();
    let idx = rand::thread_rng().gen_range(0..position_vec.len() - 1);
    let choice = position_vec[idx];

    /* randomly select from position vec */
    board.place_piece(choice.0, choice.1, agent);
}

pub fn determine_winner(board: &mut Board, agent: Agent) -> bool {
    let diagonals = diagonals(board, agent);
    let vert_horiz = vert_horiz(board, agent);

    if diagonals == true || vert_horiz == true {
        return true;
    }
    false
}

pub fn game_cycle(_rounds: i32) {
    println!("Minimax goes here");

    let mut board: Board = Board::new(6, 6);
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

    loop {
        if agent1.get_status() == true && agent2.get_status() == false {
            /* make move with minimax algo */
            take_random_action(&mut board, agent1);
            board.print_board();

            agent1.set_status(false);
            agent2.set_status(true);
        }

        if determine_winner(&mut board, agent1) {
            println!("AGENT 1 WINS! ");
            break;
        }

        if agent2.get_status() == true && agent1.get_status() == false {
            println!("Agent 2 Goes: {:?}", agent2);
            take_random_action(&mut board, agent2);
            agent2.set_status(false);
            agent1.set_status(true);
        }

        if determine_winner(&mut board, agent2) {
            println!("AGENT 2 WINS! ");
            break;
        }

        /* wait one second in between rounds */
        let second = time::Duration::from_millis(1000);
        let _now = time::Instant::now();
        thread::sleep(second);
    }
}


pub fn test_copy_board_state() {

    /* grab any two d array */ 
    let board_state = vec![vec![0,0,1], vec![2,0,2], vec![0,0,0]];
    println!("BOARD STATE: {:?}", board_state); 

    /* set up board representation */ 
    let mut board: Board = Board::new(3, 3);
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);

    /* add agents to board */
    board.add_agent(agent1);
    board.add_agent(agent2);

    /* set agent status */ 
    agent1.set_status(true);
    agent2.set_status(false);

    /* copy over board state */
    board.copy_board_state(board_state); 
    board.print_board(); 

    /* run minimax algorithm */
    let (_best_score1, _best_move1) =
        maximizer::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

    println!("BEST MOVE: {:?}", _best_move1); 

}


pub fn test_minimax() {
    println!("Minimax goes here");

    let mut board: Board = Board::new(3, 3);
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);

    /* add agents to board */
    board.add_agent(agent1);
    board.add_agent(agent2);

    println!("Added agents to board");

    agent1.set_score(0);
    agent2.set_score(0);

    let agents: &Vec<Agent> = board.get_agents();
    for a in agents {
        println!("Agent: {:?}", a);
    }

    agent1.set_status(false);
    agent2.set_status(true);

    /* test board states that make no sense here */
    board.place_piece(0, 2, agent1);
    board.place_piece(1, 1, agent2);
    board.place_piece(1, 2, agent2);

    let score = evaluation::static_evaluation(&mut board, agent1, agent2);

    /* make move with minimax algo */
    let (current_score, current_move) =
        maximizer::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

    /* place most optimal move */
    println!("Most optimal move: {:?} {:?}", current_move, current_score);
    board.print_board();
    board.place_piece(current_move.0, current_move.1, agent1);
    println!("=========================");
    board.print_board();
    println!("Score: {:?}", score);
}


pub fn test_negamax() {

    let mut board: Board = Board::new(3, 3);
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);

    /* add agents to board */
    board.add_agent(agent1);
    board.add_agent(agent2);

    println!("Added agents to board");

    agent1.set_score(0);
    agent2.set_score(0);

    let agents: &Vec<Agent> = board.get_agents();
    for a in agents {
        println!("Agent: {:?}", a);
    }

    agent1.set_status(false);
    agent2.set_status(true);

    /* create board configuration */ 
    board.place_piece(0, 2, agent1);
    board.place_piece(1, 1, agent2);
    board.place_piece(1, 2, agent2);

    /* get optimal score */ 
    let (current_score, current_move) = maximizer::negamax(
        &mut board.clone(),0,9
    );

    /* place optimal move */ 
    println!("Most optimal move: {:?} {:?}", current_move, current_score);
    board.print_board();
    board.place_piece(current_move.0, current_move.1, agent1);
    println!("=========================");
    board.print_board();



}

pub fn minimax_game_cycle(_rounds: i32) {
    println!("Minimax goes here");

    let mut board: Board = Board::new(3, 3);
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);
    let is_full = board.is_full();

    /* add agents to board */
    board.add_agent(agent1);
    board.add_agent(agent2);

    println!("Added agents to board");

    agent1.set_score(0);
    agent2.set_score(0);

    let agents: &Vec<Agent> = board.get_agents();
    for a in agents {
        println!("Agent: {:?}", a);
    }

    agent1.set_status(false);
    agent2.set_status(true);

    println!("Starting MINIMAX test.....");

    /* start game cycle here */
    while !is_full {
        board.print_board();

        let agent1_status = agent1.get_status();
        let agent2_status = agent2.get_status();

        /* users turn */
        if agent1_status == false && agent2_status == true {
            /* ask user for row */
            let mut user_row = String::new();
            println!("Enter Row: ");
            io::stdin()
                .read_line(&mut user_row)
                .expect("failed to read input.");
            let user_row: usize = user_row.trim().parse().expect("invalid input");
            println!("Entered {:?} ", user_row);

            /* ask user for col */
            let mut user_col = String::new();
            println!("Enter Row: ");
            io::stdin()
                .read_line(&mut user_col)
                .expect("failed to read input.");
            let user_col: usize = user_col.trim().parse().expect("invalid input");
            println!("Entered {:?} ", user_col);

            board.place_piece(user_row, user_col, agent2);

            agent2.set_status(false);
            agent1.set_status(true);
        }

        if agent1_status == true && agent2_status == false {
            /* make move with minimax algo */
            let (_current_score, current_move) =
                maximizer::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

            /* place most optimal move */
            println!("Most optimal move: {:?}", current_move);
            board.place_piece(current_move.0, current_move.1, agent1);

            agent1.set_status(false);
            agent2.set_status(true);
        }

        /* check if we have a winner */
        if determine_winner(&mut board, agent1) {
            println!("AGENT 1 WINS! ");
            board.print_board();
            break;
        }

        /* check if we have a winner */
        if determine_winner(&mut board, agent2) {
            println!("AGENT 2 WINS! ");
            board.print_board();
            break;
        }

        /* wait one second between each turn */
        let second = time::Duration::from_millis(1000);
        let _now = time::Instant::now();
        thread::sleep(second);
    }
}
