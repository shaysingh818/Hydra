const express = require("express");
const router = express.Router();

const GamesController = require("../controllers/GamesController");
const controller = new GamesController(); 

router.get('/setting', (req, res) => controller.viewSettings(req, res));
router.post('/setting', (req, res) => controller.createSetting(req, res));
router.put('/setting/:id', (req, res) => controller.updateSetting(req, res));
router.get('/setting/:id', (req, res) => controller.viewSetting(req, res));

module.exports = router; 
