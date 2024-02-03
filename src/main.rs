// pub mod agent;
// pub mod board;
// pub mod engine;
// pub mod game;

//! I document the current module!
pub mod ndarray;
pub mod environment;  
pub mod games;

// use crate::game::c4;
// use crate::game::tictactoe;
// use crate::ndarray::ndarray;
// use std::env;

// load the board engine

// pub fn match_command_arg(game_type: &str, rounds: &String) {
//     // convert rounds to int
//     let rnds: i32 = match rounds.parse() {
//         Ok(n) => n,
//         Err(_) => {
//             eprintln!("Not an integer");
//             0
//         }
//     };

//     match game_type {
//         "tictactoe" => {
//             tictactoe::test_minimax();
//             //tictactoe::minimax_game_cycle(rnds);
//         }
//         "c4" => {
//             c4::connect_game_cycle(rnds);
//         }

//         "tic-tac-minimax" => {
//             println!("Minimax testing goes here");
//             tictactoe::test_copy_board_state();
//         }
//         _ => {
//             println!("No game type selected");
//         }
//     }
// }

fn main() {
    println!("testing");

    for n in 2..0 {
        println!("{:?}", n); 
    }

    // match args.len() {
    //     1 => {
    //         println!("Not enough args passed");
    //     }
    //     2 => {
    //         println!("Row specific no columns");
    //     }
    //     3 => {
    //         let game = &args[1];
    //         let rounds = &args[2];
    //         match_command_arg(game, rounds);
    //     }
    //     _ => {
    //         println!("Usage: ./hydra <game_name> <rounds>");
    //     }
    // }
}
