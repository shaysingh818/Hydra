use crate::agent::Agent;
use crate::board::Board;

/*
    This file contains anything related to utilites needed for variations
    of minimaxing. This file should only store evaluation functions,
    maxamizers and minimizers and helper functions for storing data or
    passing data to the user interface.

*/

pub trait Minimax {

    /* evaluation function for regular minimax */
    fn static_evaluation(board: &mut Board, agent: Agent, opp: Agent) -> i32;

    /* negamax evaluation function */ 
    fn negmax_eval(board: &mut Board) -> i32; 

    /* maximize player moves using recursion */
    fn minimax(
        board: &mut Board,
        curr_depth: usize,
        agent: Agent,
        opp: Agent,
        root_move: (usize, usize),
        is_max: bool,
    ) -> (i32, (usize, usize));

    /* score without knowing who the player is */
    fn negamax(
        board: &mut Board, 
        curr_depth: usize,
        max_depth: usize
    ) -> (i32, (usize, usize));

    /* negamax function using alpha beta pruning */ 
    fn ab_negamax( 
        board: &mut Board, 
        curr_depth: usize,
        max_depth: usize,
        alpha: i32, 
        beta: i32
    ) -> (i32, (usize, usize));

}
