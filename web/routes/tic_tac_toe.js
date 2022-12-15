const express = require("express");
const router = express.Router();

const TicTacToeController = require("../controllers/TicTacToeController");
const controller = new TicTacToeController(); 

router.get("/game", (req, res) => controller.ttt_game(req, res));
router.post("/minimax", (req, res) => controller.minimax(req, res));

module.exports = router; 
