-- This file should undo anything in `up.sql`
-- restore all the previous data;

DROP TABLE songs;

CREATE TABLE IF NOT EXISTS songs (
    id INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    past_difficulty INTEGER,
    present_difficulty INTEGER,
    future_difficulty INTEGER -- 9+ -> 10, 10 -> 11とする
);

CREATE TABLE IF NOT EXISTS aprilfools (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL
);

INSERT INTO songs (
    title, past_difficulty, present_difficulty, future_difficulty
) VALUES 
    ("Sayonara Hatsukoi", 1, 4, 6),
    ("Fairytale", 1, 3, 7),
    ("Vexaria", 2, 5, 7),
    ("Rise", 2, 4, 7),
    ("Lucifer", 3, 5, 8),
    ("Snow White", 2, 5, 8),
    ("Shades of Light in a Transcendent Realm", 3, 6, 8),
    ("Babaroque", 3, 6, 8),
    ("Lost Civilization", 4, 7, 9),
    ("GOODTEK (Arcaea Edit)", 4, 6, 9),
    ("qualia -ideaesthesia-", 5, 7, 9),
    ("cry of viyella", 3, 6, 8),
    ("I've heard it said", 3, 6, 8),
    ("memoryfactory.lzh", 2, 5, 8),
    ("Relentless", 4, 6, 8),
    ("Lumia", 2, 6, 8),
    ("Essence of Twilight", 4, 7, 9),
    ("PRAGMATISM", 4, 8, 10),
    ("Sheriruth", 5, 7, 10),
    ("Dement ~after legend~", 3, 6, 7),
    ("Dandelion", 2, 6, 8),
    ("Infinity Heaven", 1, 5, 7),
    ("Anökumene", 2, 6, 9),
    ("Brand new world", 2, 4, 7),
    ("Paradise", 1, 4, 7),
    ("Flashback", 2, 5, 8),
    ("フライブルクとエンドロウル", 3, 6, 9),
    ("Party Vinyl", 4, 7, 9),
    ("Nirv lucE", 2, 7, 10),
    ("Chronostasis", 3, 7, 9),
    ("神奈川電脳暗渠", 1, 5, 9),
    ("DataErr0r", 3, 7, 9),
    ("CROSS†SOUL", 4, 7, 9),
    ("Your voice so... feat. Such", 3, 6, 9),
    ("クロートーと星の観測者", 2, 5, 7),
    ("Moonlight of Sand Castle", 1, 5, 7),
    ("REconstruction", 2, 6, 8),
    ("Evoltex (poppi'n mix)", 2, 7, 8),
    ("Oracle", 3, 5, 9),
    ("αterβus", 4, 7, 10),
    ("Ignotus", 3, 6, 9),
    ("不浄な白い鳥", 2, 5, 9),
    ("ハルトピア ~Utopia of Spring", 1, 4, 8),
    ("Blossoms", 1, 4, 7),
    ("vsキミ戦争", 1, 4, 7),
    ("Moonheart", 2, 5, 8),
    ("Genesis", 2, 5, 8),
    ("Lethaeus", 3, 6, 9),
    ("Auxesia", 3, 6, 9),
    ("Rabbit In The Black Room", 2, 5, 8),
    ("Modelista", 3, 7, 9),
    ("One Last Drive", 2, 5, 8),
    ("Dreamin' Attraction!!", 4, 7, 9),
    ("Red and Blue", 4, 7, 9),
    ("Iconoclast", 4, 7, 9),
    ("SOUNDWiTCH", 3, 6, 9),
    ("妖艶魔女 -trappola bewitching-", 3, 6, 9),
    ("conflict", 4, 7, 10),
    ("Axium Crisis", 5, 8, 10),
    ("Grievous Lady", 6, 9, 11),
    ("Surrender", 3, 6, 8),
    ("夜桜吹雪", 4, 7, 9),
    ("Reinvent", 2, 6, 8),
    ("Syro", 3, 6, 9),
    ("Dream goes on", 1, 5, 7),
    ("Journey", 3, 6, 8),
    ("Specta", 3, 6, 9),
    ("Quon", 4, 6, 9),
    ("cyanine", 4, 7, 10),
    ("Metalic Punisher", 3, 7, 10),
    ("Blaster", 4, 7, 9),
    ("next to you", 4, 7, 8),
    ("Silent Rush", 2, 5, 8),
    ("Strongholds", 2, 5, 9),
    ("Memory Forest", 3, 6, 9),
    ("Singularity", 4, 7, 10),
    ("carmine:scythe", 4, 7, 9),
    ("γuarδina", 4, 7, 10),
    ("Cybernecia Catharsis", 4, 7, 9),
    ("Be There", 4, 7, 9),
    ("inkar-usi", 2, 4, 7),
    ("Call My Name feat. Yukacco", 3, 6, 8),
    ("Suomi", 2, 5, 7),
    ("Bookmaker (2D Version)", 4, 6, 8),
    ("堕落の園", 2, 7, 9),
    ("九番目の迷路", 3, 3, 8),
    ("The Message", 3, 6, 9),
    ("Sulfur", 4, 6, 9),
    ("Halcyon", 5, 8, 10),
    ("Ether Strike", 5, 8, 10),
    ("Fracture Ray", 6, 9, 11),
    ("Nhelv", 3, 6, 9),
    ("dropdead", 1, 9, 8),
    ("Fallensquare", 3, 7, 9),
    ("白道、多希望羊と信じありく。", 3, 6, 9),
    ("Purgatorium", 2, 6, 8),
    ("Alexandrite", 4, 7, 9),
    ("光", 2, 6, 8),
    ("Hall of Mirrors", 3, 5, 8),
    ("STAGER (ALL STAGE CLEAR)", 3, 6, 9),
    ("Linear Accelerator", 2, 6, 9),
    ("Tiferet", 4, 7, 10),
    ("Rugie", 3, 6, 9),
    ("Astral tale", 4, 7, 9),
    ("Phantasia", 4, 5, 9),
    ("Empire of Winter", 3, 6, 9),
    ("MERLIN", 3, 5, 8),
    ("DX超性能フルメタル少女", 3, 6, 9),
    ("OMAKENO Stroke", 3, 6, 9),
    ("Scarlet Lance", 4, 7, 10),
    ("ouroboros -twin stroke of the end-", 4, 7, 10),
    ("Libertas", 3, 5, 9),
    ("虚空の夢", 4, 7, 8),
    ("Grimheart", 2, 5, 8),
    ("ReviXy", 3, 6, 8),
    ("Antithese", 2, 5, 8),
    ("Black Territory", 3, 7, 9),
    ("Corruption", 3, 6, 9),
    ("Vicious Heroism", 4, 7, 9),
    ("Cyaegha", 5, 8, 10),
    ("VECTOЯ", 3, 7, 9),
    ("SUPERNOVA", 3, 6, 9),
    ("Dot to Dot feat. shully", 3, 6, 8),
    ("Garakuta Doll Play", 4, 6, 10),
    ("怒槌", 3, 7, 10),
    ("World Vanquisher", 2, 5, 10),
    ("Dreadnought", 4, 7, 9),
    ("Particle Arts", 3, 6, 8),
    ("Vindication", 4, 6, 9),
    ("Heavensdoor", 4, 7, 9),
    ("Ringed Genesis", 5, 8, 10),
    ("Chelsea", 3, 6, 8),
    ("AI[UE]OON", 3, 6, 9),
    ("迷える音色は恋の唄", 3, 7, 9),
    ("Tie me down gently", 3, 5, 8),
    ("Valhalla:0", 4, 7, 10),
    ("Mirzam", 4, 7, 9),
    ("Diode", 2, 5, 8),
    ("FREEF4LL", 4, 7, 8),
    ("GLORY:ROAD", 4, 7, 10),
    ("Monochrome Princess", 4, 7, 9),
    ("Heavenly careless", 3, 7, 9);

INSERT INTO aprilfools ( title ) VALUES 
    ("Ignotus Afterburn"),
    ("Red and Blue and Green");
