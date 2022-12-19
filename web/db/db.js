var mysql = require('mysql');
require('dotenv').config();

var db_config = {
    host     : '127.0.0.1', 
    user     : process.env.DB_USERNAME, 
    password : process.env.DB_PASSWORD,
    database : process.env.DB_NAME
};

var connection;

function handleDisconnect() {
	
	/* create connection */ 
	connection = mysql.createConnection(db_config);

	/* connect to db */ 
	connection.connect(function(err) {
		if(err) {
			console.log("error when connection to db: ", err); 
			setTimeout(handleDisconnect, 2000); 
		}
	}); 

	/* catch errors when connecting to db */ 
	connection.on('error', function(err) {
		console.log('db error', err); 
		if(err.code == 'PROTOCOL_CONNECTION_LOST'){
			handleDisconnect(); // recursion? 
		} else {
			throw err; 
		}
	}); 
	
}

handleDisconnect();
module.exports = connection;