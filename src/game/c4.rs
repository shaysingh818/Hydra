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


pub fn check_vert_horiz(my_board: &Board, piece: i32) -> bool {

    // vars
    let board_matrix = my_board.get_board();
	let rows = my_board.get_rows(); 
	let cols = my_board.get_cols(); 
    let mut row_index = 0;
    let mut vert_col_index = 0;
	let mut horiz_col_index = 0; 
	let mut start_vert_index = 0; 
	let mut end_vert_index = 0; 
	let mut start_horiz_index = 0; 
	let mut end_horiz_index = 0; 


	for row in board_matrix {

		let mut temp_horiz = false; 
		let mut temp_vert = false;
		let mut vert_count : i32 = 0;
		let mut horiz_count = 0;  
		let mut vert_check = false; 
		let mut horiz_check = false;  

		for col in row {

			let row_value = board_matrix[vert_col_index][row_index];
			println!("Board indexes: {:?} : {:?}", row_index, vert_col_index);
			println!("Value: {:}", col); 

			if *col == piece  {
				start_horiz_index = horiz_col_index; 
				end_horiz_index = horiz_col_index + 3; 
				println!("HORIZ Checks: {:?} : {:?}", start_horiz_index, end_horiz_index);
				horiz_count = start_horiz_index as i32; 
				horiz_check = true; 
				if end_horiz_index > rows - 1 {
					println!("Horiz whoops"); 
					break; 
				}
			}
	
			if row_value == piece {
				start_vert_index = vert_col_index; 
				end_vert_index = vert_col_index + 3;
				println!("VERT Checks: {:?} : {:?}", start_vert_index, end_vert_index);
				vert_count = start_vert_index as i32;
				vert_check = true; 
				if end_vert_index > cols - 1 {
					println!("Whoops");  
					break; 
				} 
			}

			if horiz_check == true {	
				if *col == piece {
					println!("HORIZ COUNT: {:}", horiz_count); 	
					if horiz_count == end_horiz_index as i32 {
						temp_horiz = true; 
						break; 
					}
					horiz_count += 1; 
				}
			}

				
			if vert_check == true && row_value == piece {
				if row_value == piece {
					println!("VERT COUNT: {:}", vert_count); 	
					if vert_count == end_vert_index as i32 {
						temp_vert = true; 
						break; 
					}
					vert_count += 1; 
				}
			}


			if horiz_check == true && *col != piece {
				horiz_count = 0; 
				horiz_check = false; 
			}

			if vert_check == true && row_value != piece {
				vert_count = 0; 
				vert_check = false; 
			}


			if vert_col_index < my_board.get_cols() - 1 {
				vert_col_index += 1;
			} 

			horiz_col_index += 1; 

		}

		if temp_horiz == true || temp_vert == true {
			return true;  
		} 

		row_index += 1; 
		vert_col_index = 0;
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

	//println!("Rows: {:?}", row_counter); 

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

    /* randomly select from position vec */
    println!("Choice: {:?} {:?}", choice.0, choice.1);

    my_board.place_piece(choice.0, choice.1, agent.get_piece());
}



pub fn connect_game_cycle(rounds: i32){

	println!("Playing {:?} rounds of c4", rounds); 
    let mut my_board : Board = Board::new(7, 6);

    // add players
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);
	
    agent1.set_status(true);

	// create a board configuration
	my_board.place_piece(2, 2, 1);
	my_board.place_piece(3, 2, 1);
	my_board.place_piece(4, 2, 1);
	my_board.place_piece(5, 2, 1);
	my_board.print_board(); 

    let mut result = check_vert_horiz(&my_board, 1);
	println!("RESULT: {:}", result);
 

	/*
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


	} */
}


#[cfg(test)]
mod c4_tests {

	use crate::Board; 
	use crate::c4::*; 

	#[test]
    fn test_left_right_diagonal() {

        let mut board : Board = Board::new(7, 6);
        board.place_piece(0, 0, 1);
        board.place_piece(1, 1, 1);
        board.place_piece(2, 2, 1);
        board.place_piece(3, 3, 1);

        let result = check_diagonals(&board, 1);

        board.print_board();
        assert_eq!(result, true);
    }

	#[test]
    fn test_right_left_diagonal() {

        let mut board : Board = Board::new(7, 6);
        board.place_piece(0, 3, 1);
        board.place_piece(1, 2, 1);
        board.place_piece(2, 1, 1);	
        board.place_piece(3, 0, 1);

        let result = check_diagonals(&board, 1);

        board.print_board();
        assert_eq!(result, true);
    }

	
	#[test]
    fn test_verticals() {

        let mut board : Board = Board::new(7, 6);
		board.place_piece(0, 2, 1);
		board.place_piece(1, 2, 1);
		board.place_piece(2, 2, 1);
		board.place_piece(3, 2, 1);

        let mut result = check_vert_horiz(&board, 1);
        board.print_board();
		println!("result: {:?}", result); 
        assert_eq!(result, true);

		// test offset
		board.clear(); 		
		board.place_piece(1, 2, 1);
		board.place_piece(2, 2, 1);
		board.place_piece(3, 2, 1);
		board.place_piece(4, 2, 1);
	
        result = check_vert_horiz(&board, 1);
        board.print_board();
		println!("result: {:?}", result); 
        assert_eq!(result, true);
	
		// test offset again
		board.clear(); 		
		board.place_piece(2, 2, 1);
		board.place_piece(3, 2, 1);
		board.place_piece(4, 2, 1);
		board.place_piece(5, 2, 1);
	
        result = check_vert_horiz(&board, 1);
        board.print_board();
		println!("result: {:?}", result); 
        assert_eq!(result, true);

		

    }

	




}


