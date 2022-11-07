
use crate::game::tictactoe::*; 
use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time}; 
use rand::*; 
use std::env;


pub fn static_evaluation(board: &mut Board, agent: Agent) -> i32{

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


pub fn minimax_f(
	board: &mut Board, 
	curr_depth: usize, 
	agent: Agent
) -> (i32, (usize, usize)) {
	
	/* if game is over or terminal state is reached stop recursing */
	let rows = board.get_rows(); 
	let max_score = (rows * 10) as i32;
	let terminal_state = static_evaluation(board, agent) == max_score;
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
		let score = static_evaluation(board, agent); 
		if score > best_score {
			best_score = score;
			best_move = (play.0, play.1);
		}
	
		board.print_board(); 
		board.pop_piece();  	 
	}

	/* make highest scoring move and save board state */ 
	board.place_piece(
		best_move.0, 
		best_move.1, 
		agent
	);

	/* recurse and send next board state */ 
	let (current_score, current_move) = minimax_f(
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



pub fn minimax_game_cycle() {

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

	board.print_board(); 

	loop {

		if agent1.get_status() == true && agent2.get_status() == false {

			/* agent 1 uses minimaxing algorithm */
			let (current_score, current_move) = minimax_f(
				&mut board,
				0,
				agent1
			);
			
			println!("Most optimal move: {:?}", current_move);  
			board.place_piece(current_move.0, current_move.1, agent1); 
	
            agent1.set_status(false);
            agent2.set_status(true);
        }

        // check if agent 1 wins
        let diag_a1 = check_diagonals(&board, agent1.get_piece());
        if diag_a1 == true {
            println!("Agent 1 diag true");
            break;
        }

        if agent2.get_status() == true && agent1.get_status() == false {
            println!("Agent 2 Goes: {:?}", agent2);
            take_random_action(&mut board, agent2);
            agent2.set_status(false);
            agent1.set_status(true);
        }

		// check if agent 2 wins    
        let diag_a2 = check_diagonals(&board, agent2.get_piece());
        if diag_a2 == true {
            println!("Agent 2 diag true");
            break;
        }

        board.print_board();
        println!("Diag 1: {:?}", diag_a1);
        println!("Diag 2: {:?}", diag_a2);

		// wait one second
        let second = time::Duration::from_millis(1000);
        let _now = time::Instant::now();
        thread::sleep(second);


	}

}





