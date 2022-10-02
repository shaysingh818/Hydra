use crate::board::{Board};
use crate::agent::Agent;
use std::{thread, time};
use rand::Rng;


pub fn get_available_positions(my_board: &mut Board) -> Vec<(usize, usize)> {
    let mut position_vec = Vec::new();
    let board_matrix = my_board.get_board();

    let mut row_counter = 0;
    for row in board_matrix {
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



pub fn take_random_action(my_board: &mut Board, agent: Agent) {

    let position_vec = get_available_positions(my_board);
    let idx = rand::thread_rng().gen_range(0..position_vec.len() - 1);
    let choice = position_vec[idx];

    /* randomly select from position vec */
    println!("Choice: {:?} {:?}", choice.0, choice.1);


    my_board.place_piece(choice.0, choice.1, agent.get_piece());
}




pub fn connect_game_cycle(rounds: i32){

    // create board
    let mut my_board : Board = Board::new(7, 6);

    // add players
    let mut agent1: Agent = Agent::new(0);
    let mut agent2: Agent = Agent::new(1);

    my_board.add_agent(agent1);
    my_board.add_agent(agent2);

	 // loop through agents
    let agents : &Vec<Agent> = my_board.get_agents();
    for a in agents {
        println!("Agent: {:?}", a);
    }

    agent1.set_status(true);

	loop {

		if agent1.get_status() == true && agent2.get_status() == false {
            take_random_action(&mut my_board, agent1);
            agent1.set_status(false);
            agent2.set_status(true);
        }

        if agent2.get_status() == true && agent1.get_status() == false {
            println!("Agent 2 Goes: {:?}", agent2);
            take_random_action(&mut my_board, agent2);
            agent2.set_status(false);
            agent1.set_status(true);
        }

    	my_board.print_board();

		// wait one second
        let second = time::Duration::from_millis(1000);
        let now = time::Instant::now();
        thread::sleep(second);

	}



}
