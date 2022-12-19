const express = require('express');

/* routes */ 
const hydra = require('./routes/hydra');
const games = require('./routes/games'); 
const models = require('./routes/models'); 
const tictactoe = require('./routes/tic_tac_toe');


const app = express(); 
const port = 8080; 

/* serve static files and allow body parsers */ 
app.use(express.static('public'));
app.use(express.urlencoded());
app.use(express.json());
app.set('view engine', 'ejs');

/* routing */ 
app.get("/", (req, res) => res.render("index"));
app.use("/hydra", hydra);
app.use("/games", games); 
app.use("/models", models); 
app.use("/tictactoe", tictactoe); 






app.listen(port, () => {

  	/* use swagger */
    console.log(`Example app listening at http://localhost:${port}`)
})
