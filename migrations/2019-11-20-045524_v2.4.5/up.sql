-- Your SQL goes here
DELETE FROM songs WHERE title IN ("Heavenly careless");

INSERT INTO songs (
    title, difficulty, level_val
) VALUES
    ("Heavenly caress", 0, 3),
    ("Heavenly caress", 1, 7),
    ("Heavenly caress", 2, 9),
    ("Senkyou", 0, 3),
    ("Senkyou", 1, 5),
    ("Senkyou", 2, 8),
    ("Filament", 0, 4),
    ("Filament", 1, 7),
    ("Filament", 2, 9);
