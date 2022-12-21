const express = require('express');

/* routes */ 
const hydra = require('./routes/hydra');
const games = require('./routes/games'); 
const models = require('./routes/models'); 
const users = require('./routes/users'); 
const entitlements = require('./routes/entitlements'); 
const tictactoe = require('./routes/tic_tac_toe');

/* init app */ 
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
app.use("/entitlements", entitlements); 
app.use("/users", users); 
app.use("/tictactoe", tictactoe); 


app.listen(port, () => {
    console.log(`Example app listening at http://localhost:${port}`)
})
