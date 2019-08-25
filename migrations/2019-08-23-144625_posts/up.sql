-- Your SQL goes here
CREATE TABLE IF NOT EXISTS songs (
    id INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    past_difficulty NOT NULL INTEGER,
    present_difficulty NOT NULL INTEGER,
    future_difficulty NOT NULL INTEGER, -- 9+ -> 10, 10 -> 11とする
);

CREATE TABLE IF NOT EXISTS aprilfools (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL
);
