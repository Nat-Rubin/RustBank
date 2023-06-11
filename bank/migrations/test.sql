SELECT 'here' AS '';

CREATE SCHEMA IF NOT EXISTS bank

SELECT 'here2' AS '';

CREATE TABLE IF NOT EXISTS bank.users (
            ID int UNIQUE PRIMARY KEY,
            Balance int NOT NULL,
            Username varchar NOT NULL,
            Password varchar NOT NULL,
            Email varchar NOT NULL,
            Firstname varchar NOT NULL,
            Lastname varchar NOT NULL
)
            