-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    fname VARCHAR(100) NOT NULL,
    sname VARCHAR(100) NOT NULL,
    lname VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL UNIQUE,
    dob DATE,
    nationalId INTEGER NOT NULL,
    username VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
