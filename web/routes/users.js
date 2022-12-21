const express = require('express'); 
const UserController  = require('../controllers/UserController');

const router = express.Router();
const controller = new UserController(); 

/* game routes */
router.get('/', (req, res) => controller.viewUsers(req, res));
router.post('/create', (req, res) => controller.createUser(req, res));
router.put('/update/:id', (req, res) => controller.updateUser(req, res));
router.get('/view/:id', (req, res) => controller.viewUser(req, res));
router.delete('/delete/:id', (req, res) => controller.deleteUser(req, res));
router.post('/assign_entitlement/:id', (req, res) => controller.assignEntitlement(req, res));
router.get('/entitlements/:id', (req, res) => controller.viewUserEntitlements(req, res));

module.exports = router;