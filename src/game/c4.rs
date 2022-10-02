use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time};
use rand::Rng;


pub fn check_diagonals(my_board: &Board, piece: i32) -> bool {
    let mut left_diag = true;
    let mut right_diag = true;
    let mut count = 0;
    let mut board_size = my_board.get_board().len();

    loop {

        if count == 4 {
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

    my_board.add_agent(agent1);
    my_board.add_agent(agent2);

	 // loop through agents
    /*let agents : &Vec<Agent> = my_board.get_agents();
    for a in agents {
        println!("Agent: {:?}", a);
    }*/

    agent1.set_status(true);
	

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
