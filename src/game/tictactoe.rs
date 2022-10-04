use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time}; 
use rand::*; 


pub fn check_diagonals(my_board: &Board, piece: i32) -> bool {

	let mut left_right = true; 
	let mut right_left = true;
	let board_matrix = my_board.get_board();  
	let board_size = my_board.get_board().len();
	let mut row_count = (my_board.get_rows() - 1) as i32;  
	let mut col_counter = 0;

	for _col in 0..board_size {
			
		if board_matrix[col_counter][row_count as usize] != piece {
			left_right = false; 
		}
	
		if board_matrix[col_counter][col_counter] != piece {
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


pub fn check_vert_horiz(my_board: &Board, piece: i32) -> bool {

	// vars
	let mut horiz = true; 
	let mut vert = true; 
	let mut row_index = 0; 
	let mut col_index = 0; 
	let board_matrix = my_board.get_board(); 

	// loop throuh board
	for row in board_matrix {
		let mut temp_horiz = true; 
		let mut temp_vert = true; 
		for col in row {
			if *col != piece {
				temp_horiz = false; 
			}
			if board_matrix[col_index][row_index] != piece {
				temp_vert = false; 
			}
			col_index += 1; 
		}
	
		if temp_horiz == true || temp_vert == true {
			break; 
		} else {
			horiz = false; 
			vert = false; 
		}

		row_index += 1; 
		col_index = 0; 	
	}

	if horiz == true || vert == true {
		return true
	}

	return false
}

pub fn get_available_positions(my_board: &mut Board) -> Vec<(usize, usize)> {
	let mut position_vec = Vec::new(); 
	let board_matrix = my_board.get_board(); 
	
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


pub fn take_random_action(my_board: &mut Board, agent: Agent) {

	let position_vec = get_available_positions(my_board); 
	let idx = rand::thread_rng().gen_range(0..position_vec.len() - 1);
	let choice = position_vec[idx]; 

	/* randomly select from position vec */ 
	println!("Choice: {:?} {:?}", choice.0, choice.1); 
	
	
	my_board.place_piece(choice.0, choice.1, agent.get_piece()); 
}	


pub fn game_cycle(_rounds: i32) {


	// create board
	let mut my_board : Board = Board::new(3, 3);
	
	
	// add agents to the board
	let mut agent1 : Agent = Agent::new(1); 
	let mut agent2 : Agent = Agent::new(2);

	// set their score
	agent1.set_score(0); 
	agent2.set_score(0);  

	my_board.add_agent(agent1);	
	my_board.add_agent(agent2);

	// loop through agents
	let agents : &Vec<Agent> = my_board.get_agents(); 
	for a in agents {
		println!("Agent: {:?}", a); 
	}

	agent1.set_status(true);

	

	// agent toggling
	loop {
		
		if agent1.get_status() == true && agent2.get_status() == false {
			take_random_action(&mut my_board, agent1); 
			agent1.set_status(false);   
			agent2.set_status(true); 
		}

		// check if agent 1 wins
        let diag_a1 = check_diagonals(&my_board, agent1.get_piece());
        if diag_a1 == true {
            println!("Agent 1 diag true");
			break; 
        }

		if agent2.get_status() == true && agent1.get_status() == false {
			println!("Agent 2 Goes: {:?}", agent2);
			take_random_action(&mut my_board, agent2); 
			agent2.set_status(false);  
			agent1.set_status(true);  
		}
	
		// check if agent 2 wins    
        let diag_a2 = check_diagonals(&my_board, agent2.get_piece());
        if diag_a2 == true {
            println!("Agent 2 diag true");
			break; 
        }

		my_board.print_board(); 
		println!("Diag 1: {:?}", diag_a1);
		println!("Diag 2: {:?}", diag_a2);

		// wait one second
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
		board.place_piece(0, 0, 1); 	
		board.place_piece(1, 1, 1); 	
		board.place_piece(2, 2, 1); 

		let result = check_diagonals(&board, 1);

		board.print_board(); 
		assert_eq!(result, true);  
	}

	
	#[test]
	fn test_right_left_diagonal() {
		
		let mut board : Board = Board::new(3, 3); 
		board.place_piece(0, 2, 1); 	
		board.place_piece(1, 1, 1); 	
		board.place_piece(2, 0, 1); 

		let result = check_diagonals(&board, 1);

		board.print_board(); 
		assert_eq!(result, true);  
	}

	
	#[test]
	fn test_verticals() {
		
		let mut board : Board = Board::new(3, 3); 
		board.place_piece(0, 0, 1); 	
		board.place_piece(1, 0, 1); 	
		board.place_piece(2, 0, 1); 

		let result = check_vert_horiz(&board, 1);

		board.print_board(); 
		assert_eq!(result, true);  
	}

	
	#[test]
	fn test_horizontals() {
		
		let mut board : Board = Board::new(3, 3); 
		board.place_piece(0, 0, 1); 	
		board.place_piece(0, 1, 1); 	
		board.place_piece(0, 2, 1); 

		let result = check_vert_horiz(&board, 1);

		board.print_board(); 
		assert_eq!(result, true);  
	}



}

