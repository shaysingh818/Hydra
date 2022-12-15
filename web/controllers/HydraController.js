
class HydraController {

	/* games page */ 
	games(req, res) {
		res.render("games"); 
	}

	/* documentation page */ 
	docs(req, res) {
		res.render("documentation"); 
	}

}


module.exports = HydraController; 
