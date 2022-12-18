const {
	createGame,
	updateGame,
	viewGames,
	viewGame,
	deleteGame,
	createSetting,
	viewSettings,
	viewSetting,
	updateSetting,
	deleteSetting,
} = require('../models/game'); 


class GamesController {

	createGame(req, res) {
		console.log("Create game goes here");
		createGame(req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewGame(req, res) {
		console.log("View game goes here");
		viewGame(req.params.id).then((result) => {
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewGames(req, res) {
		console.log("View games goes here");
		viewGames().then((result) => {
			return res.send({message: result}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateGame(req, res) {
		console.log("create settings goes here");
		updateGame(req.params.id, req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	deleteGame(req, res) {
		console.log("create settings goes here");
		deleteGame(req.params.id).then((result) => {
			return res.send({message: "Deleted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	createSetting(req, res) {
		console.log("create settings goes here");
		createSetting(req.body).then((result) => {
			return res.send({message: "Inserted settings"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewSettings(req, res) {
		viewSettings().then((result) => {
			return res.send({message: result}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewSetting(req, res) {
		viewSetting(req.params.id).then((result) => {
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateSetting(req, res) {
		console.log("DO YOU WORK?");
		updateSetting(req.params.id, req.body).then((result) => {
			return res.send({message: "Updated settings"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	deleteSetting(req, res) {
		console.log("DO YOU WORK?");
		deleteSetting(req.params.id).then((result) => {
			return res.send({message: "Deleted setting"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	/*
	addModel(req, res) {
		console.log("DO YOU WORK?");
		addModel(req.params.id).then((result) => {
			return res.send({message: "Added Model"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	} */

}

module.exports = GamesController; 
