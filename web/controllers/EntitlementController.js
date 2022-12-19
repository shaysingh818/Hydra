const {
	createEntitlement,
	updateEntitlement,
	viewEntitlement,
	viewEntitlement,
	deleteEntitlement,
} = require('../models/entitlement'); 


class EntitlementController {

	createEntitlement(req, res) {
		console.log("Create game goes here");
		createEntitlement(req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewEntitlement(req, res) {
		console.log("View game goes here");
		viewEntitlement(req.params.id).then((result) => {
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewEntitlement(req, res) {
		console.log("View games goes here");
		viewEntitlement().then((result) => {
			return res.send({message: result}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateEntitlement(req, res) {
		console.log("create settings goes here");
		updateEntitlement(req.params.id, req.body).then((result) => {
			return res.send({message: "Inserted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	deleteEntitlement(req, res) {
		console.log("create settings goes here");
		deleteGame(req.params.id).then((result) => {
			return res.send({message: "Deleted Game"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}



}

module.exports = GamesController; 
