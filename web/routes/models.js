const express = require('express'); 
const ModelsController = require('../controllers/ModelsController.js');
const MLController = require('../controllers/MLController.js');


const router = express.Router();
const controller = new ModelsController(); 
const models = new MLController(); 


/* Models database routes */
router.get('/', (req, res) => controller.viewModels(req, res));
router.post('/create', (req, res) => controller.createModel(req, res));
router.put('/update/:id', (req, res) => controller.updateModel(req, res));
router.get('/view/:id', (req, res) => controller.viewModel(req, res));
router.delete('/delete/:id', (req, res) => controller.deleteModel(req, res));

/* Routes to deploy new models */ 
router.post('/ttt_minimax', (req, res) => models.tttMinimax(req, res));
router.post('/ttt_ab_negamax', (req, res) => models.tttAbNegamax(req, res));



module.exports = router; 
