use crate::agent::Agent;

#[derive(Debug,Clone)]
pub struct Board {
	rows: usize, 
	cols: usize,
	matrix: Vec<Vec<i32>>,
	agents: Vec<Agent>
}


impl Board {

	pub fn new (rows: usize, cols: usize) -> Board {
		Board {
			rows: rows, 
			cols: cols,
			matrix: vec![vec![0; rows]; cols], 
			agents: vec![]
		}
	}

	pub fn get_board(&self) -> &Vec<Vec<i32>> {
		&self.matrix
	}

	pub fn get_cols(&self) -> usize {
		self.cols
	}

	pub fn get_rows(&self) -> usize {
		self.rows
	}

	pub fn resize_board(&mut self, set_row: usize, set_col: usize){
		self.matrix = vec![vec![0; set_row]; set_col] 
	}

	pub fn get_pos(&self, row: usize, col: usize) -> i32 {
		self.matrix[row][col]
	}

	pub fn add_agent(&mut self, agent: Agent) {
		self.agents.push(agent)
	}

	pub fn get_agents(&self) -> &Vec<Agent> {
		&self.agents
	}

	pub fn print_board(&self){
		for row in &self.matrix {
			println!("Vec: {:?}", row); 
		} 
	}

	pub fn place_piece(&mut self, set_row: usize, set_col: usize, piece: i32) {	
		self.matrix[set_row][set_col] = piece; 
	}


}

