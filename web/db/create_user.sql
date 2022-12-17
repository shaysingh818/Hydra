
/* create user with password */ 
CREATE USER 'hydra'@'localhost' IDENTIFIED WITH mysql_native_password BY '<PASS>'; 
CREATE USER 'hydra'@'%' IDENTIFIED WITH mysql_native_password BY '<PASS>'; 

/* grant all privileges */ 
GRANT ALL ON *.* TO 'hydra'@'localhost';
GRANT ALL ON *.* TO 'hydra'@'%';

FLUSH PRIVILEGES;
