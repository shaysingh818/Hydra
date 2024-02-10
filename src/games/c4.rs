use serde::{Serialize, Deserialize};
use crate::environment::agent::Agent;
use crate::environment::state::State;


/// The state structure is used for simulating environments that agents can be added to
/// # Example Usage
/// ```rust
/// use crate::hydra::environment::agent::Agent;
/// use crate::hydra::games::c4::Connect4;
///
/// // example connect4 usage

/// let mut board = Connect4::new(vec![6, 7], 2);
/// let agent1 = Agent::new(1, "agent-1");
/// let agent2 = Agent::new(2, "agent-2");
///
/// // add agents to the board
/// board.add_agent(agent1.clone()).unwrap();
/// board.add_agent(agent2.clone()).unwrap();
///
/// // drop pieces
/// board.drop_piece(agent1.id(), 0); 
/// board.drop_piece(agent1.id(), 0);
/// board.drop_piece(agent2.id(), 2); 
/// ``` 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Connect4 {
    shape: Vec<usize>,
    state: State,
    max_players: i32,
    current_action: usize,
    agent_count: i32
}



impl Connect4 {

    /// Create instance of connect4 state
    pub fn new(shape: Vec<usize>, max_players: i32) -> Connect4 {
        Self {
            shape: shape.clone(),
            state: State::new("connect-4", shape),
            max_players: max_players, 
            current_action: 0,
            agent_count: 0,
        }
    }


    /// List current agents on the connect4 board
    pub fn agents(&self) -> &Vec<Agent> {
        self.state.agents()
    }

    /// Get the inherited state structure for the connect4 board 
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Clear the connect4 board state
    pub fn clear(&mut self) {
        self.state.clear();
        self.current_action = 0; 
    }

    /// Add agent to connect4 board state 
    pub fn add_agent(&mut self, agent: Agent) -> Result<(), String> {
        
        if self.agent_count >= self.max_players {
            return Err("Max player limit hit".to_string());
        }

        self.state.add_agent(agent);
        self.agent_count += 1; 
        Ok(()) 
    }

    /// Drop piece to corresponding board column
    pub fn drop_piece(&mut self, value: i32, column: usize) {

        let row_limit = self.shape[0]; 
        let mut row_counter = 0;

        for item in 0..row_limit-1 {
            let indices = vec![item+1, column];
            if *self.state.grid().get(indices) != 0 {
                let place_idx = vec![item, column];
                self.state.place(value, place_idx.clone());
                break;
            }
            row_counter += 1; 
        }

        /* check last value in row at bottom */ 
        let idxs = vec![row_counter, column];
        if *self.state.grid().get(idxs.clone()) == 0 {
            self.state.place(value, idxs); 
        }

    }

    /// Check that agent has 4 diagonals from both directions on the board (this is not done yet)
    pub fn diagonals(&self, agent_id: i32) -> bool {
 
        let cols = self.shape[1];
        let col_limit_offset = cols-4; 
        let mut row_counter = 0;        

        for col in 0..cols-3 {

            for _row in 0..col_limit_offset {
   
                let mut diag_check = true;
                let mut temp_row = row_counter;
                let mut temp_col = col; 
                for _item in 0..4 {
                    let value = self.state.grid().get(vec![temp_row, temp_col]); 
                    if *value != agent_id {
                        diag_check = false; 
                    }
                    temp_row += 1; 
                    temp_col += 1;
                }

                if diag_check {
                    return true;
                }
                row_counter += 1;
            }
            row_counter = 0; 
        }

       /* start v.0.2.0 here */ 
        false
    }

    /// Check that agent has 4 in row horizontally on the board
    pub fn horizontals(&self, agent_id: i32) -> bool {

        let cols = self.shape[1];
        let rows = self.shape[0];
        let grid_size = self.state.grid().size();
        let mut row_counter = 0; 
        let mut col_counter = 0; 
        
        for _item in 0..grid_size {

            if row_counter == cols-1 {
                col_counter += 1;
                row_counter = 0;  
            }

            if col_counter == rows {
                break;
            }

            let horiz_limit = row_counter + 4;
            if horiz_limit <= cols {
                let mut horiz_check = true; 
                for item in row_counter..horiz_limit {
                    let value = self.state.grid().get(vec![col_counter, item]); 
                    if *value != agent_id {
                        horiz_check = false; 
                    }
                }

                if horiz_check {
                    return true; 
                }
            } 
            
            row_counter += 1; 
        }

        false
    }

    /// Check that agent has 4 in a row vertically on board
    pub fn verticals(&self, agent_id: i32) -> bool {

        let cols = self.shape[1];
        let rows = self.shape[0];
        let grid_size = self.state.grid().size();
        let mut row_counter = 0; 
        let mut col_counter = 0; 

        for _item in 0..grid_size {

            if row_counter == rows {
                col_counter += 1;
                row_counter = 0;  
            }

            if col_counter == cols {
                break;
            }

            let vert_limit = row_counter + 4;
            if vert_limit <= rows {
                let mut vert_check = true; 
                for item in row_counter..vert_limit {
                    let value = self.state.grid().get(vec![item, col_counter]); 
                    if *value != agent_id {
                        vert_check = false; 
                    }
                }
                if vert_check {
                    return true; 
                }
            } 
            
            row_counter += 1; 
        }
        false
    }

    /// Return 2 dimensional state view of the board
    pub fn state_view(&self)  -> Result<(), String> {

        if self.state.grid().rank() != 2 {
            return Err("View only works for 2d environments".to_string());
        }

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