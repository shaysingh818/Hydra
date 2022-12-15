const express = require('express'); 
const path = require('path'); 
const app = express(); 
const port = 8080; 

/* serve static files and allow body parsers */ 
app.use(express.static('public'));
app.use(express.urlencoded());
app.use(express.json());
app.set('view engine', 'ejs');

/* routing */ 
const hydra = require("./routes/hydra"); 
app.get("/", (req, res) => res.render("index"));
app.use("/hydra", hydra);

const tic_tac_toe = require("./routes/tic_tac_toe"); 
app.use("/tictactoe", tic_tac_toe); 

app.listen(port);
console.log('Server started at http://localhost:' + port);
