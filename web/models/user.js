const db = require('../db/db'); 

createUser = (data) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'INSERT INTO USER SET ?'; 

    /* add middleware calls here */ 

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


updateUser = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = [data,  id];

	var dbQuery = "UPDATE USER SET ? WHERE user_id = ?"; 

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


viewUsers = () => new Promise((resolve, reject) => {

	console.log("View Games");
	var dbQuery = 'SELECT * FROM USER'; 

	db.query(dbQuery, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


viewUser = (id) => new Promise((resolve, reject) => {

	console.log("View Game");
	var dbQuery = 'SELECT * FROM USER WHERE user_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


deleteUser = (id) => new Promise((resolve, reject) => {

	console.log("Delete Game");
	var dbQuery = 'DELETE FROM USER WHERE user_id=?'; 

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
		user: id,
		entitlement: data.entitlement_id
	}; 

	var dbQuery = "INSERT INTO USER_ENTITLEMENT SET ?"; 
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


viewUserEntitlements = (id) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var dbQuery = "SELECT * FROM USER_ENTITLEMENT WHERE user = ?"; 

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
    createUser,
    updateUser, 
    viewUsers,
    viewUser,
    deleteUser,
	assignEntitlement,
	viewUserEntitlements
}