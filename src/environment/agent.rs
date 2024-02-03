use std::fs::File;
use std::io::{BufWriter, Read, Write};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    id: i32,
    label: String,
    score: f64,
    active: bool,
}

impl Agent {

    pub fn new(set_id: i32, set_label: &str) -> Agent {
        Agent {
            id: set_id,
            label: String::from(set_label),
            score: 0.0,
            active: false,
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn set_score(&mut self, num: f64) {
        self.score = num;
    }

    pub fn add_score(&mut self, num: f64) {
        self.score += num;
    }

    pub fn status(&mut self, status: bool) {
        self.active = status;
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

