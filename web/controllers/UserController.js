const {
    createUser,
    updateUser,
    viewUsers,
    viewUser,
    deleteUser,
    assignEntitlement, 
    viewUserEntitlements
} = require('../models/user');


class UserController {

    createUser(req, res) {
        createUser(req.body).then((result) => {
            return res.send({message: "Inserted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    viewUser(req, res) {
        viewUser(req.params.id).then((result) => {
            return res.send(result[0]);
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    viewUsers(req, res) {
        viewUsers().then((result) => {
            return res.send({message: result});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    updateUser(req, res) {
        updateUser(req.params.id, req.body).then((result) => {
            return res.send({message: "Inserted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
	}

	deleteUser(req, res) {
        deleteUser(req.params.id).then((result) => {
            return res.send({message: "Deleted Game"});
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

	viewUserEntitlements(req, res) {
		viewUserEntitlements(req.params.id).then((result) => {	
			return res.send(result); 
		}).catch((e) => {
			return res.status(400).send(e); 
		}); 
	} 

}

module.exports =  UserController; 
