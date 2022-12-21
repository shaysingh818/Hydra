const express = require('express'); 
const GamesController  = require('../controllers/GamesController');

const router = express.Router();
const controller = new GamesController(); 

/* game routes */
router.get('/', (req, res) => controller.viewGames(req, res));
router.post('/create', (req, res) => controller.createGame(req, res));
router.put('/update/:id', (req, res) => controller.updateGame(req, res));
router.get('/view/:id', (req, res) => controller.viewGame(req, res));
router.delete('/delete/:id', (req, res) => controller.deleteGame(req, res));
router.post('/assign_model/:id', (req, res) => controller.assignModel(req, res));
router.get('/models/:id', (req, res) => controller.viewGameModels(req, res));
router.post('/assign_entitlement/:id', (req, res) => controller.assignEntitlement(req, res));
router.get('/entitlements/:id', (req, res) => controller.viewGameEntitlements(req, res));


/* game settings routes */ 
router.get('/setting', (req, res) => controller.viewSettings(req, res));
router.post('/setting', (req, res) => controller.createSetting(req, res));
router.put('/setting/:id', (req, res) => controller.updateSetting(req, res));
router.get('/setting/:id', (req, res) => controller.viewSetting(req, res));
router.delete('/setting/:id', (req, res) => controller.deleteSetting(req, res));

module.exports = router; 