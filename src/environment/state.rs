use std::fs::File;
use std::io::{BufWriter, Read, Write};
use serde::{Serialize, Deserialize};
use crate::environment::agent::Agent;
use crate::ndarray::ndarray::NDArray;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    name: String,
    grid: NDArray<i32>,
    agents: Vec<Agent>,
    turn: usize,
    current_action: Vec<usize>
}

impl State {

    pub fn new(name: &str, shape: Vec<usize>) -> State {
        State {
            name: String::from(name),
            grid: NDArray::new(shape).unwrap(),
            agents: vec![],
            turn: 0, 
            current_action: vec![]         
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn grid(&self) -> &NDArray<i32> {
        &self.grid
    }

    pub fn dim(&self) -> &Vec<usize> {
        &self.grid.shape()
    }

    pub fn turn(&self) -> usize {
        self.turn
    }

    pub fn agents(&self) -> &Vec<Agent> {
        &self.agents
    }

    pub fn current_action(&self) -> &Vec<usize> {
        &self.current_action
    }

    pub fn curr_agent(&self) -> &Agent {
        &self.agents()[self.turn()]
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent)
    }
 
    pub fn place(&mut self, value: i32, coords: Vec<usize>) {
        self.current_action = coords.clone(); 
        self.grid.set(coords, value).unwrap()
    }

    pub fn resize(&mut self, new_shape: Vec<usize>) {
        self.grid = NDArray::new(new_shape).unwrap();
    }

    pub fn set_turn(&mut self, idx: usize) {
        self.turn = idx;
    }

    pub fn next_agent(&mut self) {
        if self.turn() >= 0 {
            let value = self.turn() + 1; 
            self.set_turn(value); 
        } else if self.turn() == self.agents().len() - 1 {
            self.set_turn(0);
        }
    }

    pub fn prev_agent(&mut self) {
        if self.turn() == 0 {
            let value = self.agents().len() - 1;
            self.set_turn(value);
        } else if self.turn() > 0 {
            let value = self.turn() - 1; 
            self.set_turn(value); 
        } 
    }

    pub fn is_full(&self) -> bool {
        for item in self.grid.values() {
            if *item == 0 {
                return false;
            }
        } 
        true
    }

    pub fn pop_value(&mut self) {

        if self.turn == 0 {
            self.turn = self.agents.len() - 1;
        } else {
            self.turn -= 1; 
        }

        self.grid.set(self.current_action().to_vec(), 0).unwrap();
    }

    pub fn clear(&mut self) {
        let curr_shape = self.grid.shape();
        self.grid = NDArray::new(curr_shape.to_vec()).unwrap();
    }

    pub fn print(&self) {
        println!("{:?}", self.grid.values());
    }

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


