const { viewGames } = require('../models/game'); 

class HydraController {

	/* documentation page */ 
	tictactoe(req, res) {
		res.render("games/tictactoe"); 
	}

	connect4(req, res) {
		res.render("games/c4"); 
	}

	twentyfortyeight(req, res) {
		res.render("games/2048"); 
	}

	sudoku(req, res) {
		res.render("games/sudoku"); 
	}

	colorcell(req, res) {
		res.render("games/colorcell"); 
	}

	/* games page */ 
	games(req, res) {
		/* render games here */ 
		viewGames().then((result) => {

			/* This view renders the list of games I want to write RL algorithms for */ 
			let indexCounter = 0; 
			let firstRow = result.slice(0, 3); 
			let secondRow = result.slice(3, 6) 
			let games1 = []; 
			let games2 = []; 

			firstRow.map(item => {
				let value = {
					name: item.name,
					description: item.description,
					deployment_route: item.deployment_route
				};
				games1.push(value);  
			}); 

			secondRow.map(item => {
				let value = {
					name: item.name,
					description: item.description,
					deployment_route: item.deployment_route
				};
				games2.push(value);  
			}); 

			console.log(games1); 

			return res.render("games", {
				games1: games1,
				games2: games2
			}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	/* documentation page */ 
	docs(req, res) {
		res.render("documentation"); 
	}

}

module.exports =  HydraController; 
