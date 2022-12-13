extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


pub mod board; 
pub mod agent;
pub mod engine; 
pub mod game; 


use crate::board::Board;
use crate::agent::Agent; 
use crate::engine::minimax::*;


#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}


#[wasm_bindgen]
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BoardState {
    rows: usize, 
    cols: usize,
    matrix: Vec<Vec<i32>>
}


#[wasm_bindgen]
pub fn hydra_minimax(board_state: JsValue) -> JsValue {

    let board_state: BoardState = serde_wasm_bindgen::from_value(board_state).unwrap();

    /* binding to invoke minimax function from pure rust library */ 
    let mut board: Board = Board::new(3, 3);
    let mut agent1: Agent = Agent::new(1);
    let mut agent2: Agent = Agent::new(2);

    /* add agents to board */
    board.add_agent(agent1);
    board.add_agent(agent2);

    agent1.set_status(true);
    agent2.set_status(false);


    /* copy web assembly board state to our library */
    let my_board = board_state.matrix; 
    board.copy_board_state(my_board); 
    println!("Added agents to board");


    /* call minimax function */
    let (current_score, current_move) = Board::minimax(
        &mut board.clone(), 0, 
        agent1, agent2,
        (0, 0), true
    ); 

    /* render optimal move to browser */
    let optimal_move = (current_move.0 as u8, current_move.1 as u8); 
    serde_wasm_bindgen::to_value(&optimal_move).unwrap()
}


