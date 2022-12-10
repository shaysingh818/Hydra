use crate::agent::Agent;

#[derive(Debug, Clone)]
pub struct Board {
    rows: usize,
    cols: usize,
    curr_pos: (usize, usize),
    matrix: Vec<Vec<i32>>,
    agents: Vec<Agent>,
    current_turn: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        Board {
            rows: rows,
            cols: cols,
            curr_pos: (0, 0),
            matrix: vec![vec![0; rows]; cols],
            agents: vec![],
            current_turn: 0,
        }
    }

    pub fn get_board(&self) -> &Vec<Vec<i32>> {
        &self.matrix
    }

    pub fn set_board(&mut self, new_matrix: Vec<Vec<i32>>) {
        self.matrix = new_matrix; 
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn clear(&mut self) {
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

    pub fn get_agent_current_turn(&self) -> Agent {
        self.agents[self.current_turn]
    }

    pub fn print_board(&self) {
        for row in &self.matrix {
            println!("Vec: {:?}", row);
        }
    }

    pub fn resize_board(&mut self, set_row: usize, set_col: usize) {
        self.rows = set_row;
        self.cols = set_col;
        self.matrix = vec![vec![0; set_row]; set_col]
    }

    pub fn pop_piece(&mut self) {
        /* go back to current agent turn */
        if self.current_turn == 0 {
            self.current_turn = self.agents.len() - 1;
        } else {
            self.current_turn -= 1;
        }

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

    pub fn available_moves(&mut self) -> Vec<(usize, usize)> {
        let mut position_vec = Vec::new();
        let mut row_counter = 0;
        for row in &self.matrix {
            let mut col_counter = 0;
            for col in row {
                if *col == 0 {
                    position_vec.push((row_counter, col_counter));
                }
                col_counter += 1;
            }
            row_counter += 1;
        }
        position_vec
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
        let mut row_index = 0;
        let mut col_index = 0;
        let mut horiz_count = 0;
        let mut vert_count = 0;

        for row in &self.matrix {
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

    /* function that alernates agents in between each move */
    pub fn make_move(&mut self, play: (usize, usize)) {
        let agent: Agent = self.agents[self.current_turn];
        self.place_piece(play.0, play.1, agent);

        if self.current_turn == self.agents.len() - 1 {
            self.current_turn = 0;
        } else {
            self.current_turn += 1;
        }
    }
}




#[cfg(test)]
mod board_instance {

    use crate::agent::Agent;
    use crate::board::Board;

    /* tests for the board instance */

    #[test]
    fn test_init_board() {
        let mut board: Board = Board::new(7, 6);
        let mut board_rows = board.get_rows();
        let mut board_cols = board.get_cols();

        assert_eq!(board_rows, 7);
        assert_eq!(board_cols, 6);

        board = Board::new(3, 3);
        board_rows = board.get_rows();
        board_cols = board.get_cols();

        assert_eq!(board_rows, 3);
        assert_eq!(board_cols, 3);
    }

    #[test]
    fn test_resize_board() {
        let mut board: Board = Board::new(7, 6);
        let mut board_rows = board.get_rows();
        let mut board_cols = board.get_cols();

        assert_eq!(board_rows, 7);
        assert_eq!(board_cols, 6);

        board.resize_board(3, 3);

        board_rows = board.get_rows();
        board_cols = board.get_cols();

        assert_eq!(board_rows, 3);
        assert_eq!(board_cols, 3);

        board.resize_board(4, 4);

        assert_eq!(board.get_rows(), 4);
        assert_eq!(board.get_cols(), 4);
    }

    #[test]
    fn test_pop_piece() {
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        /* iterate through agents in the board */
        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        agent1.set_status(false);
        agent2.set_status(true);

        /* test first board configuration */
        board.place_piece(0, 2, agent1);
        board.place_piece(0, 1, agent2);
        board.place_piece(1, 1, agent1);

        /* pop piece */
        board.pop_piece();
        assert_eq!(board.get_pos(1, 1), 0);
    }

    #[test]
    fn test_place_piece() {
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        /* iterate through agent list */
        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        /* set agent status */
        agent1.set_status(false);
        agent2.set_status(true);

        /* test pieces are placed on board */
        board.place_piece(0, 2, agent1);
        assert_eq!(board.get_pos(0, 2), 1);

        board.place_piece(0, 1, agent2);
        assert_eq!(board.get_pos(0, 1), 2);

        board.place_piece(1, 1, agent1);
        assert_eq!(board.get_pos(1, 1), 1);
    }

    /* these tests need to work before we can implement negamaxing */
    #[test]
    fn test_make_move() {

        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        /* iterate through agent list */
        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
 
        /* set agent status */
        agent1.set_status(false);
        agent2.set_status(true);

        /* make a set of moves and check board state */
        board.make_move((1,1));
        assert_eq!(board.get_pos(1, 1), 1);

        board.make_move((1,2));
        assert_eq!(board.get_pos(1, 2), 2);
 
        board.make_move((1,0));
        assert_eq!(board.get_pos(1, 0), 1);
 
        board.make_move((2,1));
        assert_eq!(board.get_pos(2, 1), 2);
    }

    #[test]
    fn test_add_agent_to_board() {

        let mut board: Board = Board::new(3, 3);

        /* add 1000 agents */ 
        for item in 0..1000 {
            let mut temp_agent : Agent = Agent::new(item);
            temp_agent.set_score(0); 
            board.add_agent(temp_agent); 
        }

        /* check number of agents on board */  
        assert_eq!(board.get_agents().len(), 1000);

        /* test mutability */ 
        for item in 0..10 {
            let mut temp_agent : Agent = Agent::new(item);
            temp_agent.set_score(0); 
            board.add_agent(temp_agent); 
        }

        /* check number of agents on board */  
        assert_eq!(board.get_agents().len(), 1010);

    }

    #[test]
    fn test_agent_turn_cycle() {

        /* create board instance */ 
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        /* iterate through agent list */
        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }
 
        /* make first move */
        let mut curr_turn = board.get_agent_current_turn(); 
        assert_eq!(curr_turn.get_piece(), 1);
        board.make_move((1,1));
        assert_eq!(board.get_pos(1,1), 1);

        curr_turn = board.get_agent_current_turn();  
        assert_eq!(curr_turn.get_piece(), 2);
        board.make_move((0,0));
        assert_eq!(board.get_pos(0,0), 2);

        curr_turn = board.get_agent_current_turn();  
        assert_eq!(curr_turn.get_piece(), 1);
        board.make_move((1,0));
        assert_eq!(board.get_pos(1,0), 1);

    }

    #[test]
    fn test_board_availability() {
        
        let mut board: Board = Board::new(3, 3);
        let mut agent1: Agent = Agent::new(1);
        let mut agent2: Agent = Agent::new(2);

        /* add agents to board */
        board.add_agent(agent1);
        board.add_agent(agent2);
        agent1.set_score(0);
        agent2.set_score(0);

        /* iterate through agent list */
        let agents: &Vec<Agent> = board.get_agents();
        for a in agents {
            println!("Agent: {:?}", a);
        }

        /* make first move */ 
        board.make_move((1,1));
        assert_eq!(board.get_pos(1,1), 1);
        let mut moves  = vec![(0,0), (0,1), (0,2), (1,0), (1,2), (2,0), (2,1), (2,2)];  
        assert_eq!(moves, board.available_moves());

        board.make_move((0,0));
        assert_eq!(board.get_pos(0,0), 2);
        moves  = vec![(0,1), (0,2), (1,0), (1,2), (2,0), (2,1), (2,2)];  
        assert_eq!(moves, board.available_moves());
 
        board.make_move((1,0));
        assert_eq!(board.get_pos(1,0), 1);
        moves  = vec![(0,1), (0,2), (1,2), (2,0), (2,1), (2,2)];  
        assert_eq!(moves, board.available_moves());
    }
}
