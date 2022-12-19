const {
    createUser,
    updateUser,
    viewUser,
    viewUser,
    deleteUser,
} = require('../models/user');


class UserController {

    createUser(req, res) {
        console.log("Create Model");
        createUser(req.body).then((result) => {
            return res.send({message: "Inserted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    viewUser(req, res) {
        console.log("View model goes here");
        viewUser(req.params.id).then((result) => {
            return res.send({message: result});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    viewUsers(req, res) {
        console.log("View models goes here");
        viewUsers().then((result) => {
            return res.send({message: result});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    updateUser(req, res) {
        console.log("Update model goes here");
        updateUser(req.params.id, req.body).then((result) => {
            return res.send({message: "Inserted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
	}

	deleteUser(req, res) {
        console.log("Delete model goes here");
        deleteUser(req.params.id).then((result) => {
            return res.send({message: "Deleted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

}

module.exports =  UserController; 
