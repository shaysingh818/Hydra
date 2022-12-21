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
	assignModel,
	viewGameModels,
	assignEntitlement, 
	viewGameEntitlements
} = require('../models/game'); 


class GamesController {

	createGame(req, res) {
		createGame(req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewGame(req, res) {
		viewGame(req.params.id).then((result) => {
			return res.send(result[0]); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewGames(req, res) {
		viewGames().then((result) => {
			return res.send({message: result}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateGame(req, res) {
		updateGame(req.params.id, req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	deleteGame(req, res) {
		deleteGame(req.params.id).then((result) => {
			return res.send({message: "Deleted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	createSetting(req, res) {
		createSetting(req.body).then((result) => {
			return res.send({message: "Inserted settings"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewSettings(req, res) {
		viewSettings().then((result) => {
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewSetting(req, res) {
		viewSetting(req.params.id).then((result) => {
			return res.send(result[0]); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateSetting(req, res) {
		updateSetting(req.params.id, req.body).then((result) => {
			return res.send({message: "Updated settings"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	deleteSetting(req, res) {
		deleteSetting(req.params.id).then((result) => {
			return res.send({message: "Deleted setting"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	assignModel(req, res) {
		assignModel(req.params.id, req.body).then((result) => {
			return res.send({message: "Added Model"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	} 

	viewGameModels(req, res) {
		viewGameModels(req.params.id).then((result) => {	
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	} 

	assignEntitlement(req, res) {
		assignEntitlement(req.params.id, req.body).then((result) => {
			return res.send({message: "Added entitlement to game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	} 

	viewGameEntitlements(req, res) {
		viewGameEntitlements(req.params.id).then((result) => {	
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	} 

}

module.exports = GamesController; 
