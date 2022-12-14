use crate::agent::Agent; 
use crate::board::Board; 
use crate::game::tictactoe::*; 


/* helper functions for creating evaluation functions */
pub fn diagonal_counter(board: &mut Board, agent: Agent) -> (i32, i32, i32, i32) {

    let mut lr_diag_count = 0;
    let mut rl_diag_count = 0;
    let mut opp_lr_diag_count = 0;
    let mut opp_rl_diag_count = 0;
    let mut row_count = (board.get_rows() - 1) as i32;
    let mut col_count = 0;

    let board_matrix = board.get_board();
    for _col in board_matrix {

        let lr_value = board_matrix[col_count][row_count as usize];
        let rl_value = board_matrix[col_count][col_count as usize];
        let agent_value = agent.get_piece();

        if lr_value == agent_value {
            lr_diag_count += 1;
        }

        if rl_value == agent_value {
            rl_diag_count += 1;
        }

        if lr_value != 0 && lr_value != agent_value {
            opp_lr_diag_count += 1;
        }

        if rl_value != 0 && rl_value != agent_value {
            opp_rl_diag_count += 1;
        }

        col_count += 1;
        row_count -= 1;
    }

    (rl_diag_count, lr_diag_count, opp_lr_diag_count, opp_rl_diag_count)

}


pub fn horiz_counter(board: &mut Board, agent: Agent) -> (i32, i32, i32, i32) {

    let mut row_index = 0;
    let mut col_index = 0;
    let mut horiz_count = 0;
    let mut vert_count = 0;
    let mut opp_horiz_count = 0;
    let mut opp_vert_count = 0;
    let board_matrix = board.get_board();

    for row in board_matrix {

        let mut temp_vert_count = 0;
        let mut temp_horiz_count = 0;
        let mut temp_opp_horiz_count = 0;
        let mut temp_opp_vert_count = 0;

        for col in row {

            let agent_value = agent.get_piece();
            let vertical_value = board_matrix[col_index][row_index];

            if *col == agent_value {
                temp_horiz_count += 1;
            }

            if vertical_value == agent_value {
                temp_vert_count += 1;
            }

            if *col != agent_value && *col != 0 {
                temp_opp_horiz_count += 1;
            }

            if vertical_value != agent_value && vertical_value != 0 {
                temp_opp_vert_count += 1;
            }

            col_index += 1;
        }

         if temp_vert_count > vert_count {
            vert_count = temp_vert_count;
        }

        if temp_horiz_count > horiz_count {
            horiz_count = temp_horiz_count;
        }

        if temp_opp_vert_count > opp_vert_count {
            opp_vert_count = temp_vert_count;
        }

        if temp_opp_horiz_count > opp_horiz_count {
            opp_horiz_count = temp_horiz_count;
        }

        row_index += 1;
        col_index = 0;

    }

    (horiz_count, vert_count, opp_vert_count, opp_horiz_count)
}




pub fn static_evaluation(board: &mut Board, agent: Agent, opp: Agent) -> i32 {
    /* get board dimensions */
    let mut score = 0;

    /* check if board has diagonals or verticals */
    let diag = diagonals(board, agent);
    let verts = vert_horiz(board, agent);
    let opp_diag = diagonals(board, opp);
    let opp_verts = vert_horiz(board, opp);

    if diag || verts {
        score = 10;
    }

    if opp_diag || opp_verts {
        score = -10;
    }

    score
}


pub fn negmax_eval(board: &mut Board) -> i32 {

    let mut score = 0;
    let row_count = board.get_rows() as i32;
    let curr_turn = board.get_agent_current_turn();
    let diag = diagonal_counter(board, curr_turn);
    let verts = horiz_counter(board, curr_turn);

    let agent_diag = diag.0 == row_count || diag.1 == row_count;
    let agent_vert = verts.0 == row_count || verts.1 == row_count;
    let opp_diag = diag.2 == row_count || diag.3 == row_count;
    let opp_vert = verts.2 == row_count || verts.3 == row_count;

    if agent_diag || agent_vert {
        score = 10;
    }

    if opp_diag || opp_vert {
        score = -10;
    }

    score

}



