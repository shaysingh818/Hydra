const express = require('express'); 
const TicTacToeController = require('../controllers/TicTacToeController.js'); 

const router = express.Router();
const controller = new TicTacToeController(); 

router.get("/game", (req, res) => controller.ttt_game(req, res));
router.post("/minimax", (req, res) => controller.minimax(req, res));

module.exports = router; 

