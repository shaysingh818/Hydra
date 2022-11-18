use crate::agent::Agent;
use crate::board::Board;
use crate::engine::minimax::*;
use rand::*;
use std::io;
use std::{thread, time};

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
    println!("Choice: {:?} {:?}", choice.0, choice.1);
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

impl Minimax for Board {
    fn static_evaluation(board: &mut Board, agent: Agent, opp: Agent) -> i32 {
        /* get board dimensions */
        let mut score = 0;

        /* check if board has diagonals or verticals */
        let diag = diagonals(board, agent);
        let verts = vert_horiz(board, agent);
        let opp_diag = diagonals(board, opp);
        let opp_verts = vert_horiz(board, opp);

        if diag || verts {
            score = 10;
        }

        if opp_diag || opp_verts {
            score = -10;
        }

        score
    }

    fn minimax(
        board: &mut Board,
        curr_depth: usize,
        agent: Agent,
        opp: Agent,
        root_move: (usize, usize),
        is_max: bool,
    ) -> (i32, (usize, usize)) {
        /* if there's a winner, stop searching */
        let winner = determine_winner(board, agent);
        let game_over = board.is_full();

        /* if game is over or agent wins */
        if winner || game_over {
            let score = Self::static_evaluation(board, agent, opp);
            return (score, root_move);
        }

        /* define local vars for function */
        let mut _best_score = 0;
        let mut best_move = (0, 0);

        /* switch best scores depending on the player */
        if is_max {
            _best_score = -1000
        } else {
            _best_score = 1000;
        }

        /* go through each available move */
        for play in board.available_moves() {
            /* make move */
            if is_max {
                board.place_piece(play.0, play.1, agent);
            } else {
                board.place_piece(play.0, play.1, opp);
            }

            /* recurse to the next state */
            let (current_score, _current_move) = Board::minimax(
                &mut board.clone(),
                curr_depth + 1,
                agent,
                opp,
                play,
                !is_max,
            );

            /* determine the best move and score */
            if is_max {
                if current_score > _best_score {
                    _best_score = current_score;
                    best_move = play;
                }
            } else {
                if current_score < _best_score {
                    _best_score = current_score;
                    best_move = play;
                }
            }

            board.pop_piece();
        }
        (_best_score, best_move)
    }

    fn negamax(board: &mut Board, curr_depth: usize) -> (i32, (usize, usize)) {
        /* if game is over or terminal state is reached stop recursing */
        println!("Depth: {:?}", curr_depth);
        board.print_board();

        (0, (0, 0))
    }
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
    board.place_piece(1, 1, agent1);

    board.place_piece(0, 1, agent2);
    board.place_piece(1, 0, agent2);

    let score = Board::static_evaluation(&mut board, agent1, agent2);

    /* make move with minimax algo */
    let (current_score, current_move) =
        Board::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

    /* place most optimal move */
    println!("Most optimal move: {:?} {:?}", current_move, current_score);
    board.print_board();
    board.place_piece(current_move.0, current_move.1, agent1);
    println!("=========================");
    board.print_board();
    println!("Score: {:?}", score);
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
                Board::minimax(&mut board.clone(), 0, agent1, agent2, (0, 0), true);

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
