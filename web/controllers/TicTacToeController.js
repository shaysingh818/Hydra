const { hydra_minimax , add } = require('../../pkg/hydra.js');


class TicTacToeController {

	minimax(req, res) {

  		/* Use functions instead of structs for this */
		let form_data = req.body; 
		/* pass json parameters to hydra lib */ 
		let data = {
			rows: form_data.rows, 
			cols: form_data.cols, 
			matrix: form_data.board_state
		};

		/* retrive result */ 
		let result = hydra_minimax(data);

		/* in the future, this info can be queried from a database */ 
		res.json({
			row: result[0], 
			col: result[1], 
		}); 
	}

	ttt_game(req, res) {
		res.render('minimax'); 
	}
}


module.exports = TicTacToeController; 
