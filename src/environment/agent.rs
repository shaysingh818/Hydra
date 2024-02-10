use std::fs::File;
use std::io::{BufWriter, Read, Write};
use serde::{Serialize, Deserialize};


/// The agent structure is used for simulating agents in an environment
/// # Example Usage
/// ```rust
/// use crate::hydra::environment::state::State;
/// use crate::hydra::environment::agent::Agent;
///
/// let mut state  = State::new("testing-add-agent", vec![3, 3]);
///
/// // example usage of agent structure
/// let agent1 = Agent::new(1, "agent-1");
/// let agent2 = Agent::new(2, "agent-2");
///
/// // Add agents to state structure
/// state.add_agent(agent1);
/// state.add_agent(agent2);
/// ``` 
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    id: i32,
    label: String,
    score: f64,
    active: bool,
}


/// Utility methods that every Agent instance has
impl Agent {

    /// Create instance of Agent, provide ID and label 
    pub fn new(set_id: i32, set_label: &str) -> Agent {
        Agent {
            id: set_id,
            label: String::from(set_label),
            score: 0.0,
            active: false,
        }
    }

    /// Checks if agent is active and can take action in environment
    pub fn active(&self) -> bool {
        self.active
    }

    /// Character label to identify agent
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Numerical identifer for agent
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the current score value of the agent
    pub fn score(&self) -> f64 {
        self.score
    }

    /// Set score value of agent
    pub fn set_score(&mut self, num: f64) {
        self.score = num;
    }

    /// Increment current agent score by provided numerical value
    pub fn add_score(&mut self, num: f64) {
        self.score += num;
    }

    /// Set status of agent, active = agent can take action
    pub fn status(&mut self, status: bool) {
        self.active = status;
    }


    /// Save current agent structure to json file
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


    /// Load agent structure from json value, serialize to Agent structure
    pub fn load(filepath: &str) -> std::io::Result<Agent> {
        let filename_format = format!("{filepath}.json");
        let mut file = match File::open(filename_format) {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let instance: Agent = serde_json::from_str(&contents)?;
        Ok(instance)
    }

}

