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
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct BoardState {
    rows: usize, 
    cols: usize,
    max_size: usize,
    optimal_move: (u8, u8), 
    matrix: Vec<Vec<i32>>
}


#[wasm_bindgen]
impl BoardState {

    pub fn new(set_rows: usize, set_cols: usize) -> BoardState {
        BoardState {
            rows: set_rows, 
            cols: set_cols, 
            max_size: 6,
            optimal_move: (0, 0), 
            matrix: vec![vec![0; set_rows]; set_cols]
        }
    }


    pub fn minimax(&mut self) -> JsValue {

        /* binding to invoke minimax function from pure rust library */ 
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);

        /* copy web assembly board state to our library */
        board.set_board(self.matrix.clone()); 
        println!("Added agents to board");

        /* set scores for agents */ 
        agent1.set_score(0);
        agent2.set_score(0);

        /* set player status for all */
        agent1.set_status(false);
        agent2.set_status(true);


        /* call minimax function */
        let (current_score, current_move) = Board::minimax(
            &mut board.clone(), 0, 
            agent1, agent2,
            (0, 0), true
        ); 

        /* render optimal move to browser */
        self.optimal_move = (current_move.0 as u8, current_move.1 as u8); 
        serde_wasm_bindgen::to_value(&self.optimal_move).unwrap()
    }

    pub fn render(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.matrix).unwrap()
    }

}


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
