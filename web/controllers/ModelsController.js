const {
    createModel,
    updateModel,
    viewModels,
    viewModel,
    deleteModel,
} = require('../models/model.js');

const { hydra_minimax, add } = require("../../pkg/hydra.js");

class ModelsController {

    createModel(req, res) {
        console.log("Create Model");
        createModel(req.body).then((result) => {
            return res.send({message: "Inserted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    viewModel(req, res) {
        console.log("View model goes here");
        viewModel(req.params.id).then((result) => {
            return res.send({message: result});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    viewModels(req, res) {
        console.log("View models goes here");
        viewModels().then((result) => {
            return res.send({message: result});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

    updateModel(req, res) {
        console.log("Update model goes here");
        updateModel(req.params.id, req.body).then((result) => {
            return res.send({message: "Inserted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
	}

	deleteModel(req, res) {
        console.log("Delete model goes here");
        deleteModel(req.params.id).then((result) => {
            return res.send({message: "Deleted Game"});
        }).catch((e) => {
            return res.status(400).send(e);
        });
    }

	/* deploy models here */ 
	ttt_minimax(req, res) {

        /* Use functions instead of structs for this */
        let form_data = req.body;

        let data = {
            rows: form_data.rows, 
            cols: form_data.cols, 
            matrix: form_data.board_state
        };

        /* retrive result */ 
        let result = hydra_minimax(data);

        /* in the future, this info can be queried from a database */ 
        return res.json({
            row: result[0], 
            col: result[1], 
        }); 
    }

}

module.exports =  ModelsController; 
