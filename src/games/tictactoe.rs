use serde::{Serialize, Deserialize};
use crate::environment::agent::Agent;
use crate::environment::state::State;


/// The state structure is used for simulating environments that agents can be added to
/// # Example Usage
/// ```rust
/// use crate::hydra::environment::agent::Agent;
/// use crate::hydra::games::tictactoe::TicTacToe;
///
/// // example tic tac toe usage
/// let mut board = TicTacToe::new(vec![3, 3], 2);
/// let agent1 = Agent::new(1, "agent-1");
/// let agent2 = Agent::new(2, "agent-2");
///
/// // Add agents to the board
/// board.add_agent(agent1.clone()).unwrap();
/// board.add_agent(agent2.clone()).unwrap();
///
/// // Place peices on the board
/// board.place_piece(agent1.id(), vec![0, 0]); 
/// board.place_piece(agent1.id(), vec![1, 1]);
/// board.place_piece(agent1.id(), vec![2, 2]);
///
/// // Confirm that diagonals rule has been met
/// let result = board.diagonals(agent1.clone());
/// assert_eq!(result, true);
/// ``` 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicTacToe {
    shape: Vec<usize>,
    state: State,
    max_players: i32,
    current_action: Vec<usize>,
    curr_agent_count: i32
}


impl TicTacToe {

    /// Create new instance of TicTacToe board
    pub fn new(shape: Vec<usize>, max_players: i32) -> TicTacToe {
        Self {
            shape: shape.clone(),
            state: State::new("tictactoe", shape),
            max_players: max_players, 
            current_action: vec![],
            curr_agent_count: 0,
        }
    }


    /// Retreive the max amount of players for the board environment
    pub fn max_players(&self) -> i32 {
        self.max_players
    }

    /// Get the current action that's been placed on the board
    pub fn curr_action(&self) -> &Vec<usize> {
        &self.current_action
    }

    /// Retrieve the current turn of the agent list
    pub fn curr_turn(&self) -> usize {
        self.state.turn()
    }

    /// Retrieve list of agents on the board
    pub fn players(&self) -> &Vec<Agent> {
        self.state.agents()
    }

    /// Retrieve inherited state structure for board environment
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Print current state structure
    pub fn print(&self) {
        println!("{:?}", self.state.print()); 
    }

    /// Place piece on tic tac toe board environment
    pub fn place_piece(&mut self, value: i32, coords: Vec<usize>) {
        self.state.place(value, coords.clone());
        self.current_action = coords; 
    }

    /// Remove last piece placed on the board
    pub fn pop_piece(&mut self) {
        self.state.prev_agent(); 
        self.state.place(0, self.current_action.clone()); 
    }

    /// Clear board state
    pub fn clear(&mut self) {
        self.state.clear();
        self.current_action = vec![0, 0]
    }

    /// Add agent to current board state
    pub fn add_agent(&mut self, agent: Agent) -> Result<(), String> {
        if self.curr_agent_count == self.max_players {
            return Err("Max players reached for tictactoe".to_string())
        }
        self.state.add_agent(agent);
        self.curr_agent_count += 1; 
        Ok(()) 
    }

    /// Check if agent has any diagonals from each direction
    pub fn diagonals(&self, agent: Agent) -> bool {

        let row_idx = self.shape[0]-1;
        let col_idx = 0;
        let mut row_count = row_idx as i32; 
        let mut col_count = col_idx as i32;  
        let mut left_right = true;
        let mut right_left = true;

        /* validate all dims are equal */ 
        let dim_condition = self.shape[0] != self.shape[1];
        let rank_condition = self.state.grid().rank() != 2; 

        if dim_condition || rank_condition {
            panic!("Dimensions not equal or rank is not equal to 2");
        }

        for _col in 0..self.shape[0] {

            let left_right_check = self.state.grid().get(vec![
                col_count as usize, 
                row_count as usize
            ]);

            let right_left_check = self.state.grid().get(vec![
                col_count as usize, 
                col_count as usize
            ]);

            if *left_right_check != agent.id() {
                left_right = false; 
            }

            if *right_left_check != agent.id() {
                right_left = false; 
            }

            col_count += 1; 
            row_count -= 1; 
        }

        if left_right == true || right_left == true {
            return true;
        }
        false
    }


    /// Check if agent has any verticals or horiztontals in a row across the board
    pub fn vert_horiz(&self, agent: Agent) -> bool {

        let mut vert = true; 
        let mut horiz =  true; 
        let mut row_idx = 0; 
        let mut col_idx = 0; 
        let grid_size = self.state.grid().size();
        let dim_size = self.shape[0]; 

        for _row in 0..grid_size {

            if col_idx == dim_size {
                col_idx = 0;
                row_idx += 1; 
                
                if horiz {
                    return horiz;
                }

                if vert {
                    return vert; 
                }

                vert = true; 
                horiz = true; 
            }

            let horiz_check = self.state.grid().get(vec![
                row_idx as usize, 
                col_idx as usize
            ]);

            let vert_check = self.state.grid().get(vec![
                col_idx as usize, 
                row_idx as usize
            ]);


            if *horiz_check != agent.id() {
                horiz = false; 
            }

            if *vert_check != agent.id() {
                vert = false; 
            }

            if row_idx == dim_size {
                row_idx = 0;
                col_idx += 1; 
            }

            col_idx += 1; 
        }

        horiz
    }

    /// Determine if agent has won using tictactoe rules
    pub fn winner(&self, agent: Agent) -> bool {

        let diagonals = self.diagonals(agent.clone());
        let row_col = self.vert_horiz(agent.clone());

        if diagonals || row_col {
            return true;
        }
        false
    }

    /// Determine the reward of the agent and factor in opponents environment state
    pub fn static_evaluation(&self, agent: Agent, opp: Agent) -> Result<i32, String> {
        
        let mut score = 0;
        if self.state.agents().len() > 2 {
            return Err("Support for only 2 agents for static evaluation".to_string())
        }

        let diag = self.diagonals(agent.clone());
        let verts = self.vert_horiz(agent.clone());
        let opp_diag = self.diagonals(opp.clone());
        let opp_verts = self.vert_horiz(opp.clone());

        if diag || verts {
            score = 10;
        }

        if opp_diag || opp_verts {
            score = -10;
        }

        Ok(score)
    }


}



