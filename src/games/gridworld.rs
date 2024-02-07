use serde::{Serialize, Deserialize};
use crate::environment::agent::Agent;
use crate::environment::state::State;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GridWorld {
    shape: Vec<usize>,
    state: State,
    max_players: i32,
    agent_count: i32, 
    curr_pos: Vec<usize>,
    start_pos: Vec<usize>,
    end_positions: Vec<Vec<usize>>
}


impl GridWorld {

    pub fn new(shape: Vec<usize>, max_players: i32) -> GridWorld {
        GridWorld {
            shape: shape.clone(),
            state: State::new("grid-world", shape),
            max_players: max_players, 
            agent_count: 0,
            curr_pos: vec![],
            start_pos: vec![],
            end_positions: vec![]
        }
    }

    pub fn start_pos(&self) -> &Vec<usize> {
        &self.start_pos 
    }

    pub fn end_positions(&self) -> &Vec<Vec<usize>> {
        &self.end_positions
    }

    pub fn agents(&self) -> &Vec<Agent> {
        self.state.agents()
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn set_start_pos(&mut self, start_pos: Vec<usize>, value: i32) {
        self.curr_pos = start_pos.clone();
        self.start_pos = start_pos.clone();
        self.state.place(value, start_pos); 
    }

    pub fn set_end_pos(&mut self, end_pos: Vec<usize>, reward: i32) {
        self.end_positions.push(end_pos.clone());
        self.state.place(reward, end_pos);
    }

    pub fn take_action(&mut self, coords: Vec<usize>, value: i32) {
        self.state.place(0, self.curr_pos.clone()); 
        self.curr_pos = coords.clone();
        self.state.place(value, coords);    
    }

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

    pub fn add_agent(&mut self, agent: Agent) -> Result<(), String> {
        
        if self.agent_count >= self.max_players {
            return Err("Max player limit hit".to_string());
        }

        self.state.add_agent(agent);
        self.agent_count += 1; 
        Ok(()) 
    }

    pub fn avaliable_moves(&self) -> Result<Vec<Vec<usize>>, String> {

        if self.state.grid().rank() != 2 {
            return Err("Method only works on rank 2 environments".to_string())
        }

        let row_dim = self.shape[0] as i32-1;
        let col_dim = self.shape[1] as i32-1; 
        let up = self.curr_pos[1] as i32 + 1; 
        let down = self.curr_pos[1] as i32 - 1; 
        let left = self.curr_pos[0] as i32 + 1; 
        let right = self.curr_pos[0] as i32 - 1; 
        let mut actions = Vec::new();

        if up < row_dim && up >= 0 {
            actions.push(vec![self.curr_pos[0], up as usize]);
        }

        if down < row_dim && down >= 0 {
            actions.push(vec![self.curr_pos[0], down as usize]);
        }

        if right < col_dim && right >= 0 {
            actions.push(vec![right as usize, self.curr_pos[1]]);
        }

        if left < col_dim && left >= 0 {
            actions.push(vec![left as usize, self.curr_pos[1]]);
        }

        Ok(actions)
    }

}