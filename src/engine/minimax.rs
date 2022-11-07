
use crate::game::tictactoe::*; 
use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time}; 
use rand::*; 
use std::env;


pub fn static_evaluation(board: &mut Board, piece: i32) -> i32{


	/* get board dimensions */ 
	let rows = board.get_rows(); 
	let cols = board.get_cols(); 

	/* get values for score function */ 
	let diag_count = board.diagonal_count(piece);
	let vh_count = board.vertical_count(piece);  
	let mut score = 0; 

	/* create score distribution */ 
	println!("Distribution: ({:?}, {:?})", 0, rows * 10);  

	/* go through all possible diagonal and vertical counts */ 
	let mut count = rows as i32; 
	while(count > 0){
		println!("Count: {:?}", count);

		if diag_count.0 == count || diag_count.1 == count {
			score = count * 10;
			break; 
 
		} else if vh_count.0 == count || vh_count.1 == count {
			score = count * 10; 
			break; 
		}
 
		score -= 10; 
		count -= 1; 
	}

	score
}


pub fn minimax_f(
	board: &mut Board, 
	curr_depth: usize, 
	max_depth: usize, 
	piece: i32
) -> (i32, (usize, usize)) {
	
	/* if game is over or terminal state is reached stop recursing */
	let rows = board.get_rows(); 
	let max_score = (rows * 10) as i32;
	let terminal_state = static_evaluation(board, piece) == max_score;
	let game_over = board.is_full(); 
	
	if terminal_state || game_over {
		return (0, (0, 0)); 
	}
 
	/* go through each move */ 	
	let positions = get_available_positions(board);  

	let mut best_score = 0;
	let mut best_move = (0, 0); 
	for play in positions {

		/* generate board state */ 
		board.place_piece(play.0, play.1, 1);	
		
		/* get score */ 
		let score = static_evaluation(board, piece); 
		if score > best_score {
			best_score = score;
			best_move = (play.0, play.1);
		}
	
		board.print_board(); 
		board.pop_piece();  	 
	}

	/* make highest scoring move and save board state */ 
	board.place_piece(best_move.0, best_move.1, 1);

	/* recurse and send next board state */ 
	let (current_score, current_move) = minimax_f(
		board, 
		curr_depth + 1,
		max_depth, 
		piece 	
	);

	/* bubble up values from stack tree */ 
	if current_score > best_score {
		best_score = current_score; 
		best_move = current_move; 
	}  
		
	(best_score, best_move)
}



pub fn t_minimax() {

	println!("Minimax goes here");

	let mut board : Board = Board::new(4, 4);
	let mut agent1 : Agent = Agent::new(1); 
	let mut agent2 : Agent = Agent::new(2); 

	/* add agents to board */ 	
	agent1.set_score(0); 	
	agent2.set_score(0);
	
	let agents : &Vec<Agent> = board.get_agents(); 
	for a in agents {
		println!("Agent: {:?}", a); 
	}

	agent1.set_status(true);

	/* create random board state */ 
	//board.place_piece(0, 0, 1);     
	board.place_piece(1, 1, 1);     
	board.place_piece(0, 1, 2);     
	board.print_board();

	 
	let (current_score, current_move) = minimax_f(
		&mut board,
		0,
		10,  
		1
	);

	println!("Most optimal move: {:?}", current_move); 

	 
 	 
}





