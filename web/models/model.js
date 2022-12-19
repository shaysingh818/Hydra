const db = require('../db/db'); 


createModel = (data) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'INSERT INTO MODEL SET ?'; 

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


viewModels = () => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'SELECT * FROM MODEL'; 

	db.query(dbQuery, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
});


viewModel = (id) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'SELECT * FROM MODEL WHERE model_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 



updateModel = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = [data,  id];
	console.log("DO YOU WORK? " + insert); 

	var dbQuery = "UPDATE MODEL SET ? WHERE model_id = ?"; 

	db.query(dbQuery, insert, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
});


deleteModel = (id) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'DELETE FROM MODEL WHERE model_id=?'; 

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
	createModel,
    updateModel, 
    viewModels,
    viewModel,
    deleteGame
}