use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time}; 
use rand::Rng; 


pub fn check_diagonals(my_board: Board, piece: i32) -> bool {
	let mut left_diag = true; 
	let mut right_diag = true;
	let mut count = 0; 
	let mut board_size = my_board.get_board().len(); 

	loop {

		if count == board_size {
			break; 
		}

		if my_board.get_pos(count, board_size) != piece {
			right_diag = false; 
		} 

		if my_board.get_pos(count, count) != piece {
			left_diag = false; 
		}		

		board_size -= 1; 
		count += 1;
		
	} 

	if left_diag == true || right_diag == true {
		return true
	}

	return false

}


pub fn check_vert_horiz(my_board: Board, piece: i32) -> bool {

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


pub fn game_cycle(rounds: i32) {


	// create board
	let mut my_board : Board = Board::new(3, 3);

	// add agents to the board
	let mut agent1 : Agent = Agent::new(0); 
	let mut agent2 : Agent = Agent::new(1); 

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

		if agent2.get_status() == true && agent1.get_status() == false {
			println!("Agent 2 Goes: {:?}", agent2);
			take_random_action(&mut my_board, agent2); 
			agent2.set_status(false);  
			agent1.set_status(true);  
		}

		my_board.print_board(); 

		// wait one second
		let second = time::Duration::from_millis(1000); 
		let now = time::Instant::now(); 
		thread::sleep(second); 	

	}
}
