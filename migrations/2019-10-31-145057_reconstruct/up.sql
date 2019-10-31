-- Your SQL goes here

CREATE TABLE IF NOT EXISTS songs_ (
    id INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    difficulty ENUM("PAST", "PRESENT", "FUTURE", "APRIL"),
    level_val INTEGER  -- 9+ -> 10, 10 -> 11, April fool -> 0
);

INSERT INTO songs_ (title, difficulty, level_val)
    SELECT title, "PAST", past_difficulty FROM songs;

INSERT INTO songs_ (title, difficulty, level_val)
    SELECT title, "PRESENT", present_difficulty FROM songs;

INSERT INTO songs_ (title, difficulty, level_val)
    SELECT title, "FUTURE", future_difficulty FROM songs;

INSERT INTO songs_ (title, difficulty, level_val) 
    SELECT title, "APRIL", 0 FROM aprilfools;

DROP TABLE songs;
DROP TABLE aprilfools;
ALTER TABLE songs_ RENAME TO songs;
