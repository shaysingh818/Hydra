use std::cmp; 

use crate::agent::Agent; 
use crate::board::Board; 
use crate::game::tictactoe::*; 
use crate::engine::minimax_test::tictactoe::*; 



pub fn minimax(
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
        let score = evaluation::static_evaluation(board, agent, opp);
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
        let (current_score, _current_move) = minimax(
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

pub fn negamax(
    board: &mut Board, 
    curr_depth: usize,
    max_depth: usize
) -> (i32, (usize, usize)) {


    /* if game is over or terminal state is reached stop recursing */
    let game_over = board.is_full();
    let curr_agent = board.get_agent_current_turn(); 
    let winner = determine_winner(board, curr_agent);
    if game_over || winner { 
        let score = evaluation::negmax_eval(board);
        return (score, (0, 0)); 
    }

    /* define local vars for function */
    let mut best_score = -1000;
    let mut best_move = (0, 0);

    /* go through each available move */
    for play in board.available_moves() {

        /* make move */ 
        board.make_move(play);

        /* recurse to the next state */
        let (recursed_score, _current_move) = negamax(
            &mut board.clone(),
            curr_depth + 1,
            max_depth
        );

        /* determine best score */ 
        let current_score = -recursed_score; 
        if current_score > best_score {
            best_score = current_score; 
            best_move = play; 
        }

    
        /* undo move */ 
        board.pop_piece(); 

    }


    (best_score, best_move)
}

pub fn ab_negamax( 
    board: &mut Board, 
    curr_depth: usize,
    max_depth: usize,
    alpha: i32, 
    beta: i32
) -> (i32, (usize, usize)) {

    /* if game is over or terminal state is reached stop recursing */
    let game_over = board.is_full();
    let curr_agent = board.get_agent_current_turn(); 
    let winner = determine_winner(board, curr_agent);
    if game_over || winner { 
        let score = evaluation::negmax_eval(board);
        return (score, (0, 0)); 
    }

    /* define local vars for function */
    let mut best_score = -1000;
    let mut best_move = (0, 0);

    /* go through each available move */
    for play in board.available_moves() {

        /* make move */ 
        board.make_move(play);

        /* get alpha value */ 
        let alpha_value = cmp::max(alpha, best_score); 

        /* recurse to the next state */
        let (recursed_score, _current_move) = ab_negamax(
            &mut board.clone(),
            curr_depth + 1,
            max_depth,
            -beta,
            -alpha_value
        );

        /* determine best score */ 
        let current_score = -recursed_score; 
        if current_score > best_score {
            best_score = current_score; 
            best_move = play; 
        
            /* if we're outside bounds, prune the tree */
            if best_score > beta {
                return (best_score, best_move); 
            }
        }

        /* undo move */ 
        board.pop_piece(); 

    }

    (best_score, best_move) 
}