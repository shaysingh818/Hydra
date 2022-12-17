USE hydra; 

CREATE TABLE USER (
    user_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) UNIQUE,
    password VARCHAR(255),
    email VARCHAR(255),
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    date_created DATETIME DEFAULT NULL
);


CREATE TABLE SETTINGS (
    settings_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    board_rows INT,
    board_cols INT,
    default_piece_value INT
);

CREATE TABLE ENTITLEMENT (
    entitlement_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    date_created DATETIME DEFAULT NULL
);

CREATE TABLE GAME (
    game_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    description VARCHAR(255),
    deployment_route VARCHAR(255),
    plays INT,
    favorites INT,
    date_created DATETIME DEFAULT NULL,
    settings INT,
    FOREIGN KEY(settings) REFERENCES SETTINGS(settings_id)
);


CREATE TABLE MODEL (
    model_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    endpoint_route VARCHAR(255),
    invocations INT,
    favorites INT,
    wins INT,
    losses INT,
    date_created DATETIME DEFAULT NULL
);

/* add relations here */
CREATE TABLE USER_ENTITLEMENT (
    user_entitlement_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    user INT,
    entitlement INT,
    FOREIGN KEY(user) REFERENCES USER(user_id),
    FOREIGN KEY(entitlement) REFERENCES ENTITLEMENT(entitlement_id)
);


CREATE TABLE GAME_ENTITLEMENT (
    game_entitlement_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    game INT,
    entitlement INT,
    FOREIGN KEY(game) REFERENCES GAME(game_id),
    FOREIGN KEY(entitlement) REFERENCES ENTITLEMENT(entitlement_id)
);

CREATE TABLE GAME_MODEL (
    game_model_id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    game INT,
    model INT,
    FOREIGN KEY(game) REFERENCES GAME(game_id),
    FOREIGN KEY(model) REFERENCES MODEL(model_id)
);
