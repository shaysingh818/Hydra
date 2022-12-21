const {
	createEntitlement,
	updateEntitlement,
	viewEntitlements,
	viewEntitlement,
	deleteEntitlement,
} = require('../models/entitlement'); 


class EntitlementController {

	createEntitlement(req, res) {
		console.log("Create game goes here");
		createEntitlement(req.body).then((result) => {
			return res.send({message: "Created Entitlement"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewEntitlement(req, res) {
		console.log("View game goes here");
		viewEntitlement(req.params.id).then((result) => {
			return res.send(result[0]); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	viewEntitlements(req, res) {
		console.log("View games goes here");
		viewEntitlements().then((result) => {
			return res.send({message: result}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	updateEntitlement(req, res) {
		console.log("create settings goes here");
		updateEntitlement(req.params.id, req.body).then((result) => {
			return res.send({message: "Updated Entitlement"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}

	deleteEntitlement(req, res) {
		console.log("create settings goes here");
		deleteEntitlement(req.params.id).then((result) => {
			return res.send({message: "Deleted Entitlement"}); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	}



}

module.exports = EntitlementController; 
