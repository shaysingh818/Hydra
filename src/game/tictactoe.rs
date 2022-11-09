use crate::board::{Board};
use crate::agent::Agent;
use crate::engine::minimax::*; 
use std::{thread, time}; 
use rand::*; 


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

	// check final result
	if left_right == true || right_left == true {
		return true
	}

	false
}



pub fn vert_horiz(board: &Board, agent: Agent) -> bool {

	// vars
	let mut horiz = false; 
	let mut vert = false;  
	let mut row_index = 0; 
	let mut col_index = 0; 
	let board_matrix = board.get_board(); 

	for row in board_matrix {
		let mut temp_horiz = true; 
		let mut temp_vert = true; 
		for col in row {
			if *col != agent.get_piece() {
				temp_horiz = false; 
			}
			if board_matrix[col_index][row_index] != agent.get_piece() {
				temp_vert = false; 
			}
			col_index += 1;
		}

		if temp_horiz == true || temp_vert == true {
			if temp_horiz {
				horiz = true; 
			}

			if temp_vert {
				vert = true; 
			}
			break; 
		} 

		row_index += 1; 
		col_index = 0; 	
	}

	if horiz == true || vert == true {
		return true
	}
	false
}



pub fn get_available_positions(board: &mut Board) -> Vec<(usize, usize)> {
	let mut position_vec = Vec::new(); 
	let board_matrix = board.get_board(); 

	let mut row_counter = 0; 
	for row in board_matrix {
		let mut col_counter = 0; 
		for col in row {
			if *col == 0 {
				position_vec.push((row_counter, col_counter));
			}
			col_counter += 1; 
		}
		row_counter += 1; 
	}
	position_vec
}


pub fn take_random_action(board: &mut Board, agent: Agent) {

	let position_vec = get_available_positions(board); 
	let idx = rand::thread_rng().gen_range(0..position_vec.len() - 1);
	let choice = position_vec[idx]; 

	/* randomly select from position vec */ 
	println!("Choice: {:?} {:?}", choice.0, choice.1); 	
	board.place_piece(choice.0, choice.1, agent); 
}



pub fn determine_winner(board: &mut Board, agent: Agent) -> bool{

	let diagonals = diagonals(board, agent);
	let vert_horiz = vert_horiz(board, agent); 

	if diagonals == true || vert_horiz == true {
		return true; 
	}
	false 
}


pub fn game_cycle(_rounds: i32) {

	println!("Minimax goes here");

	let mut board : Board = Board::new(6, 6);
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

	
	fn static_evaluation(board: &mut Board, agent: Agent) -> i32{

		/* get board dimensions */ 
		let rows = board.get_rows(); 
		let cols = board.get_cols(); 

		/* get values for score function */ 
		let diag_count = board.diagonal_count(agent);
		let vh_count = board.vertical_count(agent);  
		let mut score = 0; 

		/* go through all possible diagonal and vertical counts */ 
		let mut count = rows as i32; 
		while count > 0 {

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

	
	fn minimax_f(
		board: &mut Board, 
		curr_depth: usize, 
		agent: Agent
	) -> (i32, (usize, usize)) {
			
		/* if game is over or terminal state is reached stop recursing */
		let rows = board.get_rows(); 
		let max_score = (rows * 10) as i32;
		let terminal_state = Self::static_evaluation(board, agent) == max_score;
		let game_over = board.is_full(); 
			
		/* base case */ 
		if terminal_state || game_over {
			return (0, (0, 0)); 
		}
		 
		/* go through each move */ 	
		let positions = get_available_positions(board);  
		let mut best_score = 0;
		let mut best_move = (0, 0); 
		for play in positions {

			/* generate board state */ 
			board.place_piece(play.0, play.1, agent);	
				
			/* get score */ 
			let score = Self::static_evaluation(board, agent); 
			if score > best_score {
				best_score = score;
				best_move = (play.0, play.1);
			}

			board.pop_piece();  	 		
			
		}

		/* make highest scoring move and save board state */ 
		board.place_piece(
			best_move.0, 
			best_move.1, 
			agent
		);

		/* recurse and send next board state */ 
		let (current_score, current_move) = Self::minimax_f(
			board, 
			curr_depth + 1,
			agent
		);
			
		/* bubble up values from stack tree */ 
		if current_score > best_score {
			best_score = current_score; 
			best_move = current_move; 
		}

		(best_score, best_move)
	}

	fn negamax_eval(board: &mut Board) -> i32 {
		println!("Negamax eval goes here"); 
		1
	}


	fn negamax(board: &mut Board, curr_depth: usize) -> (i32, (usize, usize)) {
		println!("Negmax eval function"); 
		(0, (0, 0))
	}

}


pub fn minimax_game_cycle(rounds: i32){

	println!("Minimax goes here");

    let mut board : Board = Board::new(6, 6);
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

	loop {

		 if agent1.get_status() == true && agent2.get_status() == false {

			/* make move with minimax algo */
			let (current_score, current_move) = Board::minimax_f(
				&mut board.clone(), 
				0, 
				agent1
			); 
			println!("Most optimal move: {:?}", current_move); 
			board.place_piece(current_move.0, current_move.1, agent1); 
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

		board.print_board();

        let second = time::Duration::from_millis(1000);
        let _now = time::Instant::now();
        thread::sleep(second);
	}  

}


#[cfg(test)]
mod tests {

	use crate::Board;
	use crate::tictactoe::*; 

	#[test]
	fn test_left_right_diagonal() {
		
		let mut board : Board = Board::new(3, 3); 
    	let mut agent1 : Agent = Agent::new(1); 
		let mut agent2 : Agent = Agent::new(2); 

		/* add agents to board */   
		agent1.set_score(0);   
		agent2.set_score(0);
    
    	let agents : &Vec<Agent> = board.get_agents(); 
    	for a in agents {
        	println!("Agent: {:?}", a); 
    	}
		board.place_piece(0, 0, agent1); 	
		board.place_piece(1, 1, agent1); 	
		board.place_piece(2, 2, agent1); 

		let result = diagonals(&board, agent1);

		board.print_board(); 
		assert_eq!(result, true);  
	}

	
	#[test]
	fn test_right_left_diagonal() {
		
		let mut board : Board = Board::new(3, 3); 
    	let mut agent1 : Agent = Agent::new(1); 
		let mut agent2 : Agent = Agent::new(2); 

		/* add agents to board */   
		agent1.set_score(0);   
		agent2.set_score(0);
    
    	let agents : &Vec<Agent> = board.get_agents(); 
    	for a in agents {
        	println!("Agent: {:?}", a); 
    	}
		board.place_piece(0, 2, agent1); 	
		board.place_piece(1, 1, agent1); 	
		board.place_piece(2, 0, agent1); 

		let result = diagonals(&board, agent1);

		board.print_board(); 
		assert_eq!(result, true);  
	}

	
	#[test]
	fn test_verticals() {
		
		let mut board : Board = Board::new(3, 3); 
    	let mut agent1 : Agent = Agent::new(1); 
		let mut agent2 : Agent = Agent::new(2); 

		/* add agents to board */   
		agent1.set_score(0);   
		agent2.set_score(0);
    
    	let agents : &Vec<Agent> = board.get_agents(); 
    	for a in agents {
        	println!("Agent: {:?}", a); 
    	}
		board.place_piece(0, 0, agent1); 	
		board.place_piece(1, 0, agent1); 	
		board.place_piece(2, 0, agent1); 

		let result = vert_horiz(&board, agent1);

		board.print_board(); 
		assert_eq!(result, true);  
	}

	
	#[test]
	fn test_horizontals() {
		
		let mut board : Board = Board::new(3, 3); 
    	let mut agent1 : Agent = Agent::new(1); 
		let mut agent2 : Agent = Agent::new(2); 

		/* add agents to board */   
		agent1.set_score(0);   
		agent2.set_score(0);
    
    	let agents : &Vec<Agent> = board.get_agents(); 
    	for a in agents {
        	println!("Agent: {:?}", a); 
    	}
		board.place_piece(0, 0, agent1); 	
		board.place_piece(0, 1, agent1); 	
		board.place_piece(0, 2, agent1); 

		let result = vert_horiz(&board, agent1);

		board.print_board(); 
		assert_eq!(result, true);  
	}


	#[test]
	fn test_higher_dimension_boards() {

		let mut board : Board = Board::new(5, 5);
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

    	board.place_piece(4, 0, agent1);
    	board.place_piece(4, 1, agent1);
    	board.place_piece(4, 2, agent1);    
    	board.place_piece(4, 3, agent1);    
    	board.place_piece(4, 4, agent1);

		let result = vert_horiz(&board, agent1);	
		assert_eq!(result, true);  
		board.print_board(); 

	}



}

