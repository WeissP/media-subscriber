CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE,
    hashed_password CHAR(64) NOT NULL,
    subscribed_channels CHAR(24) ARRAY NOT NULL
);

