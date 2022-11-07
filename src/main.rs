pub mod agent; 
pub mod board; 
pub mod game;
pub mod engine;  

use std::env;
use crate::game::tictactoe; 
use crate::game::c4; 
use crate::board::Board;
use crate::engine::minimax;  


// load the board engine

pub fn match_command_arg(game_type: &str, rounds: &String) {
	
	// convert rounds to int
	let rnds : i32 = match rounds.parse() {
		Ok(n) => {
			n
		},
		Err(_) => {
			eprintln!("Not an integer");
			0 
		}
	}; 

	match game_type {
		"tictactoe" => {
			tictactoe::game_cycle(rnds); 
		}, 
		"c4" => {
			c4::connect_game_cycle(rnds); 
		},

		"tic-tac-minimax" => {
			println!("Minimax testing goes here");
			minimax::t_minimax();  
		},  
		_ => {
			println!("No game type selected"); 
		},
	}
}


fn main() {

	let args: Vec<String> = env::args().collect(); 
	println!("{:?}", args); 

	match args.len() {

		1 => {
			println!("Not enough args passed"); 
		}, 
		2 => {
			println!("Row specific no columns"); 
		}, 
		3 => {
			let game = &args[1]; 
			let rounds = &args[2]; 
			match_command_arg(game, rounds); 
		},
		_ => {
			println!("Usage: ./hydra <game_name> <rounds>"); 
		}, 
	}
}
