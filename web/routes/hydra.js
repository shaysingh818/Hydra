const express = require("express");
const router = express.Router();

const HydraController = require("../controllers/HydraController");
const controller = new HydraController(); 

router.get("/games", (req, res) => controller.games(req, res));
router.get("/docs", (req, res) => controller.docs(req, res));

module.exports = router; 
