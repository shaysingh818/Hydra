use crate::agent::Agent;

#[derive(Debug,Clone)]
pub struct Board {
	rows: usize, 
	cols: usize,
	curr_pos: (usize, usize), 
	matrix: Vec<Vec<i32>>,
	agents: Vec<Agent>
}


impl Board {

	pub fn new (rows: usize, cols: usize) -> Board {
		Board {
			rows: rows, 
			cols: cols,
			curr_pos: (0, 0), 
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

	pub fn clear(&mut self){	
		self.matrix = vec![vec![0; self.rows]; self.cols] 
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
	
	/* add method to remove most recently placed piece */ 
	pub fn pop_piece(&mut self) {
		let row = self.curr_pos.0; 
		let col = self.curr_pos.1; 
		self.matrix[row][col] = 0; 
	}

	pub fn place_piece(&mut self, set_row: usize, set_col: usize, agent: Agent) {	
		self.curr_pos = (set_row, set_col); 
		self.matrix[set_row][set_col] = agent.get_piece(); 
	}

	/* 
		Helper functions for creating evaluation functions. These indicate
		how close the computer is to winning the game. 
	*/

	pub fn is_full(&self) -> bool {
		for row in &self.matrix {
			for col in row {
				if *col == 0 {
					return false; 
				}
			}
		}
		true
	} 

	pub fn diagonal_count(&self, agent: Agent) -> (i32, i32) {
		
		let mut lr_diag_count = 0; 
		let mut rl_diag_count = 0; 
		let mut row_count = (self.rows - 1) as i32; 
		let mut col_count = 0; 

		for _col in &self.matrix {

			if self.matrix[col_count][row_count as usize] == agent.get_piece() {
				lr_diag_count += 1; 				
			}
	
			if self.matrix[col_count][col_count as usize] == agent.get_piece() {
				rl_diag_count += 1; 				
			}

			col_count += 1; 
			row_count -= 1; 
		}

		(rl_diag_count, lr_diag_count)	
	}


	pub fn vertical_count(&self, agent: Agent) -> (i32, i32) {

		let mut horiz = true; 
		let mut vert = true; 
		let mut row_index = 0; 
		let mut col_index = 0; 
		let mut horiz_count = 0; 
		let mut vert_count = 0; 

		for row in &self.matrix {

			let mut temp_horiz = true; 
			let mut temp_vert = true; 
			let mut temp_vert_count = 0; 
			let mut temp_horiz_count = 0; 
			for col in row {

				if *col == agent.get_piece() {
					temp_horiz_count += 1; 
				} 

				if self.matrix[col_index][row_index] == agent.get_piece() {
					temp_vert_count += 1; 
				} 
				col_index += 1;
			}


			if temp_vert_count > vert_count {
				vert_count = temp_vert_count; 
			} 

			if temp_horiz_count > horiz_count {
				horiz_count = temp_horiz_count; 
			}

			row_index += 1; 
			col_index = 0; 
			
        }

		(horiz_count, vert_count)

	}

}

