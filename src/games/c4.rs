use serde::{Serialize, Deserialize};
use crate::environment::agent::Agent;
use crate::environment::state::State;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connect4 {
    shape: Vec<usize>,
    state: State,
    max_players: i32,
    current_action: Vec<usize>,
    agent_count: i32
}



impl Connect4 {

    pub fn new(shape: Vec<usize>, max_players: i32) -> Connect4 {
        Self {
            shape: shape.clone(),
            state: State::new("connect-4", shape),
            max_players: max_players, 
            current_action: vec![],
            agent_count: 0,
        }
    }

    pub fn agents(&self) -> &Vec<Agent> {
        self.state.agents()
    }


    pub fn drop_piece(&mut self, value: i32, column: usize) {

        let row_limit = self.shape[0]; 
        let mut row_counter = 0;

        for item in 0..row_limit-1 {
            println!("[{:?} {:?}]", item+1, column);
            let indices = vec![item+1, column];
            if *self.state.grid().get(indices) != 0 {
                let place_idx = vec![item, column]; 
                self.state.place(value, place_idx);
            }
            row_counter += 1; 
        }

        /* check last value in row at bottom */ 
        let idxs = vec![row_counter, column];
        if *self.state.grid().get(idxs.clone()) == 0 {
            self.state.place(value, idxs); 
        }

    }

    pub fn diagonals(&self) -> bool {
        let mut left_right = true; 
        let mut right_left = true; 

        false
    }

    pub fn horizontals(&self) -> bool {
        false
    }

    pub fn verticals(&self) -> bool {
        false
    }

    pub fn add_agent(&mut self, agent: Agent) -> Result<(), String> {
        
        if self.agent_count >= self.max_players {
            return Err("Max player limit hit".to_string());
        }

        self.state.add_agent(agent);
        self.agent_count += 1; 
        Ok(()) 
    }

    pub fn state_view(&self)  -> Result<(), String> {

        if self.state.grid().rank() != 2 {
            return Err("View only works for 2d environments".to_string());
        }

        let values = self.state.grid().values();
        let rows = self.shape[0]; 
        let cols = self.shape[1]; 
        for row in 0..rows {
            for col in 0..cols {
                let val = self.state.grid().get(vec![row, col]);
                print!("{:?} ", val); 
            }
            println!(""); 
        }

        Ok(())
    }


}