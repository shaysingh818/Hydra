use std::fs::File;
use std::io::{BufWriter, Read, Write};
use serde::{Serialize, Deserialize};
use crate::environment::agent::Agent;
use crate::structures::ndarray::NDArray;


/// The state structure is used for simulating environments that agents can be added to
/// # Example Usage
/// ```rust
/// use crate::hydra::environment::agent::Agent;
/// use crate::hydra::environment::state::State;
///
/// // example state structure usage
/// let mut state  = State::new("testing-add-agent", vec![3, 3]);
/// let agent1 = Agent::new(1, "agent-1");
/// let agent2 = Agent::new(2, "agent-2");
///
/// // Add agents to state structure
/// state.add_agent(agent1);
/// state.add_agent(agent2);
/// ``` 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    name: String,
    grid: NDArray<i32>,
    agents: Vec<Agent>,
    turn: usize,
    current_action: Vec<usize>
}

impl State {

    /// Create new instance of multidimensional state structure
    pub fn new(name: &str, shape: Vec<usize>) -> State {
        State {
            name: String::from(name),
            grid: NDArray::new(shape).unwrap(),
            agents: vec![],
            turn: 0, 
            current_action: vec![]         
        }
    }

    /// Retrieve the name of the current state
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Retrieve the grid NDArray values of the state
    pub fn grid(&self) -> &NDArray<i32> {
        &self.grid
    }

    /// Retrieve the dimensions of the state
    pub fn dim(&self) -> &Vec<usize> {
        &self.grid.shape()
    }

    /// Retrieve the current agents turn on the state
    pub fn turn(&self) -> usize {
        self.turn
    }

    /// List all agents associated with the state environment
    pub fn agents(&self) -> &Vec<Agent> {
        &self.agents
    }

    /// Get the current action that was last placed on the state
    pub fn current_action(&self) -> &Vec<usize> {
        &self.current_action
    }

    /// Retrieve the current agent structure using the turn as an index
    pub fn curr_agent(&self) -> &Agent {
        &self.agents()[self.turn()]
    }

    /// Add instance of agent to state environment 
    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent)
    }
 
    /// Place value with multidimensional coordinates on the state 
    pub fn place(&mut self, value: i32, coords: Vec<usize>) {
        self.current_action = coords.clone(); 
        self.grid.set(coords, value).unwrap()
    }

    /// Resize dimensions of state using new shape values
    pub fn resize(&mut self, new_shape: Vec<usize>) {
        self.grid = NDArray::new(new_shape).unwrap();
    }

    /// Set the current turn cycle index for the agents on the state 
    pub fn set_turn(&mut self, idx: usize) {
        self.turn = idx;
    }

    /// Cycle to the next agent in the agent list 
    pub fn next_agent(&mut self) {
        if self.turn() >= 0 {
            let value = self.turn() + 1; 
            self.set_turn(value); 
        } else if self.turn() == self.agents().len() - 1 {
            self.set_turn(0);
        }
    }

    /// Cycle to the previous agent in the agent list 
    pub fn prev_agent(&mut self) {
        if self.turn() == 0 {
            let value = self.agents().len() - 1;
            self.set_turn(value);
        } else if self.turn() > 0 {
            let value = self.turn() - 1; 
            self.set_turn(value); 
        } 
    }

    /// Check if state is filled with agent pieces 
    pub fn is_full(&self) -> bool {
        for item in self.grid.values() {
            if *item == 0 {
                return false;
            }
        } 
        true
    }

    /// Remove last placed piece on state
    pub fn pop_value(&mut self) {

        if self.turn == 0 {
            self.turn = self.agents.len() - 1;
        } else {
            self.turn -= 1; 
        }

        self.grid.set(self.current_action().to_vec(), 0).unwrap();
    }

    /// Clear state of all places values 
    pub fn clear(&mut self) {
        let curr_shape = self.grid.shape();
        self.grid = NDArray::new(curr_shape.to_vec()).unwrap();
    }

    /// Print values on the state grid 
    pub fn print(&self) {
        println!("{:?}", self.grid.values());
    }

    /// List available slots on the board (available actions will be different for each game environment)
    pub fn actions(&mut self) -> Vec<Vec<usize>> {
        let mut counter = 0; 
        let mut result: Vec<Vec<usize>> = Vec::new();
        for item in self.grid.values() {
            if *item == 0 {
                let indices = self.grid.indices(counter).unwrap();
                result.push(indices);
            }
            counter += 1; 
        }
        result
    }

    ///  Save instance of state structure to json file
    pub fn save(&self, filepath: &str) -> std::io::Result<()> {
        let filename_format = format!("{filepath}.json");
        let file = match File::create(filename_format) {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        let mut writer = BufWriter::new(file);
        let json_string = serde_json::to_string_pretty(&self)?;
        writer.write_all(json_string.as_bytes())?;
        Ok(())
    }

    /// Serialize json file to state structure
    pub fn load(filepath: &str) -> std::io::Result<State> {
        let filename_format = format!("{filepath}.json");
        let mut file = match File::open(filename_format) {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let instance: State = serde_json::from_str(&contents)?;
        Ok(instance)
    }

}


