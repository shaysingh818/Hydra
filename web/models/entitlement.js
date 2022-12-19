const db = require('../db/db'); 

createEntitlement = (data) => new Promise((resolve, reject) => {

	console.log("Check data");
	var dbQuery = 'INSERT INTO ENTITLEMENT SET ?'; 

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


updateEntitlement = (id, data) => new Promise((resolve, reject) => {

	console.log("Check data: " + id);
	var insert = [data,  id];

	var dbQuery = "UPDATE ENTITLEMENT SET ? WHERE entitlement_id = ?"; 

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


viewEntitlements = () => new Promise((resolve, reject) => {

	console.log("View Games");
	var dbQuery = 'SELECT * FROM ENTITLEMENT'; 

	db.query(dbQuery, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


viewEntitlement = (id) => new Promise((resolve, reject) => {

	console.log("View Game");
	var dbQuery = 'SELECT * FROM ENTITLEMENT WHERE entitlement_id=?'; 

	db.query(dbQuery, id, function(err, results, fields){
		if(err) {
			console.log(err); 
			reject(); 
		} else {
			resolve(results); 
		}
	})
}); 


deleteEntitlement = (id) => new Promise((resolve, reject) => {

	console.log("Delete Game");
	var dbQuery = 'DELETE FROM ENTITLEMENT WHERE entitlement_id=?'; 

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
    createEntitlement,
    updateEntitlement, 
    viewEntitlements,
    viewEntitlement,
    deleteEntitlement
}