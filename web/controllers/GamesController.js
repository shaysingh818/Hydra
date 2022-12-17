const {
	createGame,
	updateGame,
	createSetting,
	viewSettings,
	viewSetting,
	updateSetting
} = require('../models/game');


class GamesController {

	createGame(req, res) {
		console.log("create settings goes here");
		createGame(req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
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
			return res.send({message: result}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateSetting(req, res) {
		console.log("create settings goes here");
		updateSetting(req.params.id, req.body).then((result) => {
			return res.send({message: "Updated settings"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

}

module.exports = GamesController; 
