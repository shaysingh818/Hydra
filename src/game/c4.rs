use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time};
use rand::Rng;


pub fn check_diagonals(my_board: &Board, piece: i32) -> bool {
	
    let board_matrix = my_board.get_board();
	
    let mut left_right = true;
    let mut right_left = true;
	let mut start_left : i32 = 0; 
	let mut start_right : i32 = 6;

	while start_right >= 3 && start_left <= 3 {

		let mut temp_right = start_right; 
		let mut temp_left = start_left;
		left_right = true; 
		right_left = true;  

		for _sub in 0..4 {

			if board_matrix[_sub as usize][temp_left as usize] != piece {
				left_right = false; 
			}		
				
			if board_matrix[_sub as usize][temp_right as usize] != piece {
				right_left = false; 
			}			

			temp_left += 1; 
			temp_right -= 1; 			
		} 

		if right_left == true || left_right == true {
			break; 
		}

		start_right -= 1; 
		start_left += 1; 		
	}


	if left_right == true || right_left == true {
		return true; 
	}

	false
}


pub fn check_verticals(my_board: &Board, piece: i32) -> bool {

    let board_matrix = my_board.get_board();
	let _cols = my_board.get_cols() + 1;	
	let _rows = my_board.get_rows() - 1; 
    let mut row_index = 0;
	let mut _start_index = 0; 
	let mut end_index = 0; 

	for _col in 0.._cols {	
    	let mut _vert_col_index = 0;
		let mut temp_vert = false; 
		let mut vert_check = false; 
		let mut vert_count = 0; 

		for _row in 0.._rows {

			let row_value = board_matrix[_vert_col_index][row_index];				 
			if row_value == piece && vert_check == false {
				_start_index = _vert_col_index; 
				end_index = _vert_col_index + 3;
				vert_count = _start_index as i32;
				vert_check = true; 
				if end_index > _cols - 1 {
					break; 
				} 
			}
	
			if vert_check == true && row_value == piece {
				if vert_count == end_index as i32 {
					temp_vert = true; 
					break; 
				}
				vert_count += 1; 
			}
	
			if vert_check == true && row_value != piece {
				vert_count = 0; 
				vert_check = false; 
			}

			_vert_col_index += 1; 
		}

		
		if temp_vert == true {
			return true; 
		}

		row_index += 1;
		_vert_col_index = 0;  
	}


	false

}

pub fn check_horizontals(my_board: &Board, piece: i32) -> bool {
	
    let board_matrix = my_board.get_board();
	let _rows = my_board.get_rows(); 
	let mut horiz_col_index = 0; 

	for row in board_matrix {

		let mut temp_horiz = false; 
		let mut horiz_count = 0;  
		let mut horiz_check = false;  
		let mut _start_index = 0; 
		let mut end_index = 0; 

		for _col in row {

			if *_col == piece && horiz_check == false{
				_start_index = horiz_col_index; 
				end_index = horiz_col_index + 3; 
				horiz_count = _start_index as i32; 
				horiz_check = true; 
				if end_index > _rows - 1 {
					break; 
				}
			}
	
			if horiz_check == true && *_col == piece {	
				if horiz_count == end_index as i32 {
					temp_horiz = true; 
					break; 
				}
				horiz_count += 1; 
			}

			if horiz_check == true && *_col != piece {
				horiz_count = 0; 
				horiz_check = false; 
			}

			horiz_col_index += 1; 
		}

		if temp_horiz == true {
			return true;  
		} 

		horiz_col_index = 0;  
	}


    false
}


pub fn get_available_positions(my_board: &mut Board) -> Vec<(usize, usize)> {
    let mut position_vec = Vec::new();

	let rows = my_board.get_cols(); 
    let cols = my_board.get_rows(); 
	let col_counter = cols -1; 
	println!("Board cols: {:?}", cols);
	println!("Board rows: {:?}", rows); 

	for _col in 0..col_counter {	
		let mut row_counter = (rows - 1) as i32; 
		while row_counter >= 0 {
			if my_board.get_pos(row_counter as usize, _col) == 0 {
				position_vec.push((row_counter as usize, _col));
				break;  
			}
			row_counter -= 1;			
		} 
	}

	println!("My vec: {:?}", position_vec); 

	position_vec
}


pub fn take_random_action(my_board: &mut Board, agent: Agent) {

    let position_vec = get_available_positions(my_board);
    let idx = rand::thread_rng().gen_range(0..position_vec.len() - 1);
    let choice = position_vec[idx];

    println!("Choice: {:?} {:?}", choice.0, choice.1);

    my_board.place_piece(choice.0, choice.1, agent);
}



pub fn connect_game_cycle(rounds: i32){

	println!("Playing {:?} rounds of c4", rounds); 
    let mut my_board : Board = Board::new(7, 6);

    // add players
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);
	
    agent1.set_status(true);


    my_board.add_agent(agent1);
    my_board.add_agent(agent2);	
	my_board.print_board();

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
        thread::sleep(second);
	}
}

