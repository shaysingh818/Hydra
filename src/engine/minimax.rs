
use crate::game::tictactoe::*; 
use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time}; 
use rand::*; 
use std::env;

/*
	This file contains anything related to utilites needed for variations
	of minimaxing. This file should only store evaluation functions, 
	maxamizers and minimizers and helper functions for storing data or
	passing data to the user interface. 

*/

pub trait Minimax {
	
	/* evaluate state of board from agent view */ 
	fn static_evaluation(board: &mut Board, agent: Agent) -> i32;
 
	/* maximize player moves using recursion */ 
	fn minimax(
		board: &mut Board, 
		curr_depth: usize,
		agent: Agent,
		opp: Agent,
		root_move: (usize, usize),  
		is_max: bool
	) -> (i32, (usize, usize));

	/* this one is hard */ 
	fn negamax(board: &mut Board, curr_depth: usize) -> (i32, (usize, usize)); 	
	
}

