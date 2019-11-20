-- This file should undo anything in `up.sql`
DELETE FROM songs WHERE title IN ("Heavenly caress","Senkyou", "Filament");

INSERT INTO songs (
    title, difficulty, level_val
) VALUES 
    ("Heavenly careless", 0, 3),
    ("Heavenly careless", 1, 7),
    ("Heavenly careless", 2, 9);
