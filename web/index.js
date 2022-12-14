const express = require('express'); 
const path = require('path'); 
const app = express(); 
const port = 8080; 

/* import rust library */ 
const { hydra_minimax , add } = require('../pkg/hydra.js');
//const my_board = BoardState.new(3, 3); 

/* serve static files and allow body parsers */ 
app.use(express.static('public'));
app.use(express.urlencoded());
app.use(express.json());


/* render home page  */
app.get('/', function(req, res) {
  res.sendFile(path.join(__dirname, '/public/html/index.html'));
});


/* render games page  */
app.get('/games', function(req, res) {
  res.sendFile(path.join(__dirname, '/public/html/games.html'));
});


/* render docs page  */
app.get('/docs', function(req, res) {
  res.sendFile(path.join(__dirname, '/public/html/documentation.html'));
});


/* render games detail  */
app.get('/game-detail', function(req, res) {
  res.sendFile(path.join(__dirname, '/public/html/game_detail.html'));
});

/* invoke minimax function from hydra  */
app.post('/tic-tac-toe', function(req, res) {

  	/* Use functions instead of structs for this */
	let form_data = req.body; 
	/* pass json parameters to hydra lib */ 
	let data = {
		rows: form_data.rows, 
		cols: form_data.cols, 
		matrix: form_data.board_state
	};

	/* retrive result */ 
	let result = hydra_minimax(data); 

	/* in the future, this info can be queried from a database */ 
	res.json({
		row: result[0], 
		col: result[1], 
	}); 
	
});


/* render games detail  */
app.get('/minimax', function(req, res) {
  	res.sendFile(path.join(__dirname, '/public/html/minimax.html'));
});



app.listen(port);
console.log('Server started at http://localhost:' + port);
