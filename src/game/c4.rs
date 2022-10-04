use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time};
use rand::Rng;


pub fn check_diagonals(my_board: &Board, piece: i32) -> bool {
	
    let board_matrix = my_board.get_board();
    let board_size = my_board.get_board().len();
	
    let mut left_right = true;
    let mut right_left = true;
    let mut row_count = (my_board.get_rows() - 1) as i32;
    let mut col_counter = 0;
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

	let mut rows = my_board.get_cols(); 
    let mut cols = my_board.get_rows(); 
	println!("Board cols: {:?}", cols);
	println!("Board rows: {:?}", rows); 

	let mut col_counter = cols -1; 
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

    // create board
    let mut my_board : Board = Board::new(7, 6);	
	

    // add players
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);
	
    agent1.set_status(true);

    my_board.add_agent(agent1);
    my_board.add_agent(agent2);
	my_board.print_board(); 

	let result = check_diagonals(&my_board, agent1.get_piece()); 	
	println!("Result: {:?}", result); 

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
        let now = time::Instant::now();
        thread::sleep(second);

	}
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




}


