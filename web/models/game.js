const db = require('../db/db'); 

createGame = (data) => new Promise((resolve, reject) => {

	/* default setting values */ 
	let settingsData = {
		board_rows: 3,
		board_cols: 3, 
		default_piece_value: 0
	};

	var dbQuery = 'INSERT INTO SETTINGS SET ?';
	var gameQuery = 'INSERT INTO GAME SET ?'; 
	
	db.query(dbQuery, settingsData, function(err, results, fields){
		if(err) {
			reject(); 
			console.log(err); 

		} else {
			resolve(settingsData);
			console.log(settingsData); 
			console.log(results);


			/* create game model */
			data.settings = results.insertId; 
			console.log(data);
			db.query(gameQuery, data, function(err, results, fields){
				if(err) {
					console.log(err); 
					reject(); 
				} else {
					resolve(data); 
				}
			})
		}
	})

	console.log("Executed query on db"); 
}); 


updateGame = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = [data,  id];

	var dbQuery = "UPDATE GAME SET ? WHERE game_id = ?"; 

	db.query(dbQuery, insert, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
	console.log("Update game"); 
	console.log(data); 
}); 


viewGames = () => new Promise((resolve, reject) => {

	console.log("View Games");
	var dbQuery = 'SELECT * FROM GAME'; 

	db.query(dbQuery, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


viewGame = (id) => new Promise((resolve, reject) => {

	console.log("View Game");
	var dbQuery = 'SELECT * FROM GAME WHERE game_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


deleteGame = (id) => new Promise((resolve, reject) => {

	console.log("Delete Game");
	var dbQuery = 'DELETE FROM GAME WHERE game_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
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
	var insert = [data,  id];

	var dbQuery = "UPDATE SETTINGS SET ? WHERE settings_id = ?"; 

	db.query(dbQuery, insert, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
});


deleteSetting = (id) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'DELETE FROM SETTINGS WHERE settings_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


assignModel = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = {
		game: id,
		model: data.model_id
	}; 

	var dbQuery = "INSERT INTO GAME_MODEL SET ?"; 
	console.log(dbQuery); 

	db.query(dbQuery, insert, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
	console.log("Add Model to game"); 
	console.log(data); 
}); 


viewGameModels = (id) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var dbQuery = "SELECT * FROM GAME_MODEL WHERE game = ?"; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


assignEntitlement = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = {
		game: id,
		entitlement: data.entitlement_id
	}; 

	var dbQuery = "INSERT INTO GAME_ENTITLEMENT SET ?"; 
	console.log(dbQuery); 

	db.query(dbQuery, insert, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
	console.log("Add entitlement to game"); 
	console.log(data); 
}); 


viewGameEntitlements = (id) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var dbQuery = "SELECT * FROM GAME_ENTITLEMENT WHERE game = ?"; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 




module.exports = {
	createGame,
    updateGame, 
    viewGames,
    viewGame,
    deleteGame,
	createSetting,
    updateSetting, 
    viewSettings,
    viewSetting,
    deleteSetting,
	assignModel,
	viewGameModels,
	assignEntitlement, 
	viewGameEntitlements
}
