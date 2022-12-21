const express = require('express'); 
const EntitlementController  = require('../controllers/EntitlementController');

const router = express.Router();
const controller = new EntitlementController(); 

/* game routes */
router.get('/', (req, res) => controller.viewEntitlements(req, res));
router.post('/create', (req, res) => controller.createEntitlement(req, res));
router.put('/update/:id', (req, res) => controller.updateEntitlement(req, res));
router.get('/view/:id', (req, res) => controller.viewEntitlement(req, res));
router.delete('/delete/:id', (req, res) => controller.deleteEntitlement(req, res));


module.exports = router;