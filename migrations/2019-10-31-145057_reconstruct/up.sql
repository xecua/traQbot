-- Your SQL goes here

CREATE TABLE IF NOT EXISTS songs_ (
    id INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    difficulty INTEGER NOT NULL, -- PAST: 0, PRESENT: 1, FUTURE: 2, April fool: 3 (Diesel does not support Enum)
    level_val INTEGER NOT NULL -- 9+ -> 10, 10 -> 11, April fool -> 0
);

INSERT INTO songs_ (title, difficulty, level_val)
    SELECT title, 0, past_difficulty FROM songs;

INSERT INTO songs_ (title, difficulty, level_val)
    SELECT title, 1, present_difficulty FROM songs;

INSERT INTO songs_ (title, difficulty, level_val)
    SELECT title, 2, future_difficulty FROM songs;

INSERT INTO songs_ (title, difficulty, level_val) 
    SELECT title, 3, 0 FROM aprilfools;

DROP TABLE songs;
DROP TABLE aprilfools;
ALTER TABLE songs_ RENAME TO songs;
