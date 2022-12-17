var db = require('../db/db'); 



createGame = (data) => new Promise((resolve, reject) => {

	console.log("Insert game");
	var dbQuery = 'INSERT INTO GAME SET ?'; 

	db.query(dbQuery, data, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(data); 
		}
	})
	console.log(data);
	console.log("Executed query on db"); 

}); 


updateGame = (data) => new Promise((resolve, reject) => {
	console.log("Update game"); 
	console.log(data); 
}); 

viewGames = () => new Promise((resolve, reject) => {
	console.log("View games"); 
}); 


viewGame = (data) => new Promise((resolve, reject) => {
	console.log("View game"); 
	console.log(data); 
}); 


deleteGame = (data) => new Promise((resolve, reject) => {
	console.log("delete game"); 
	console.log(data); 
});


deleteGames = () => new Promise((resolve, reject) => {
	console.log("delete games"); 
}); 


createSetting = (data) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'INSERT INTO SETTINGS SET ?'; 

	db.query(dbQuery, data, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(data); 
		}
	})
	console.log(data);
	console.log("Executed query on db"); 
}); 


viewSettings = () => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'SELECT * FROM SETTINGS'; 

	db.query(dbQuery, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


viewSetting = (id) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'SELECT * FROM SETTINGS WHERE settings_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


updateSetting = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = [data.board_rows, data_board_cols, id];

	var dbQuery = `UPDATE SETTINGS 
			SET board_rows=? 
			SET board_cols=?
		WHERE settings_id=?`; 

	db.query(dbQuery, insert, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


module.exports = {
	createSetting,
	viewSettings,
	viewSetting,
	updateSetting,
	createGame,
	updateGame,
	viewGames,
	viewGame,
	deleteGame,
	deleteGames
}; 
