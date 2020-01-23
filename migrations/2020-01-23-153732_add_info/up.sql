-- Your SQL goes here
ALTER TABLE songs ADD COLUMN pack VARCHAR(255);

-- fix typo
UPDATE songs SET title = "ハルトピア ~Utopia of Spring~" WHERE title = "ハルトピア ~Utopia of Spring";
UPDATE songs SET title = "堕楽の園" WHERE title = "堕落の園";
UPDATE songs SET title = "Metallic Punisher" WHERE title = "Metalic Punisher";

-- set pack information
UPDATE songs SET pack = "Memory Archive" WHERE title IN (
    "DataErr0r", "CROSS†SOUL", "Your voice so... feat. Such", "不浄な白い鳥", "Auxesia", "Modelista", "Surrender", "夜桜吹雪", "Metallic Punisher", "carmine:scythe", "Be There", "Call My Name feat. Yukacco", "dropdead", "Fallensquare", "Alexandrite", "Astral tale", "Phantasia", "Empire of Winter", "Libertas", "Dot to Dot feat. shully", "Dreadnought", "Mirzam", "Heavenly caress", "Filament", "Avant Raze", "BATTLE NO.1", "La'qryma of the Wasteland", "Einherjar Joker", "IZANA", "最強STRONGER"
);
UPDATE songs SET pack = "Arcaea" WHERE title IN (
    "Sayonara Hatsukoi", "Fairytale", "Vexaria", "Rise", "Lucifer", "Snow White", "Shades of Light in a Transcendent Realm", "Babaroque", "Lost Civilization", "GOODTEK (Arcaea Edit)", "qualia -ideaesthesia-", "Dement ~after legend~", "Dandelion", "Infinity Heaven", "Anökumene", "Brand new world", "Chronostasis", "神奈川電脳暗渠", "クロートーと星の観測者", "Ignotus", "ハルトピア ~Utopia of Spring~", "Rabbit In The Black Room", "One Last Drive", "Dreamin' Attraction!!", "Red and Blue", "Reinvent", "Syro", "Blaster", "Cybernecia Catharsis", "inkar-usi", "Suomi", "Bookmaker (2D Version)", "堕楽の園", "Nhelv", "白道、多希望羊と信じありく。", "Purgatorium", "Rugie", "ReviXy", "Grimheart", "VECTOЯ", "SUPERNOVA", "Diode", "FREEF4LL", "Monochrome Princess", "Senkyou"
);
UPDATE songs SET pack = "Adverse Prelude" WHERE title IN (
    "Particle Arts", "Vindication", "Heavensdoor", "Ringed Genesis"
);
UPDATE songs SET pack = "Luminous Sky" WHERE title IN (
    "九番目の迷路", "The Message", "Sulfur", "Halcyon", "Ether Strike", "Fracture Ray"
);
UPDATE songs SET pack = "Vicious Labyrinth" WHERE title IN (
    "Iconoclast", "SOUNDWiTCH", "妖艶魔女 -trappola bewitching-", "conflict", "Axium Crisis", "Grievous Lady"
);
UPDATE songs SET pack = "Eternal Core" WHERE title IN (
    "cry of viyella", "I've heard it said", "memoryfactory.lzh", "Relentless", "Lumia", "Essence of Twilight", "PRAGMATISM", "Sheriruth", "虚空の夢"
);
UPDATE songs SET pack = "Sunset Radiance" WHERE title IN (
    "Chelsea","AI[UE]OON","迷える音色は恋の唄","Tie me down gently","Valhalla:0"
);
UPDATE songs SET pack = "Absolute Reason" WHERE title IN (
    "Antithese","Black Territory","Corruption","Vicious Heroism","Cyaegha"
);
UPDATE songs SET pack = "Binary Enfold" WHERE title IN (
    "next to you","Silent Rush","Strongholds","Memory Forest","Singularity"
);
UPDATE songs SET pack = "Ambivalent Vision" WHERE title IN (
    "Blossoms","vsキミ戦争","Moonheart","Genesis","Lethaeus"
);
UPDATE songs SET pack = "Crimson Solace" WHERE title IN (
    "Paradise","Flashback","フライブルクとエンドロウル","Party Vinyl","Nirv lucE","GLORY:ROAD"
);
UPDATE songs SET pack = "CHUNITHM Collaboration" WHERE title IN (
    "Garakuta Doll Play","怒槌","World Vanquisher"
);
UPDATE songs SET pack = "Groove Coaster Collaboration" WHERE title IN (
    "MERLIN","DX超性能フルメタル少女","OMAKENO Stroke","Scarlet Lance","ouroboros -twin stroke of the end-"
);
UPDATE songs SET pack = "Tone Sphere Collaboration" WHERE title IN (
    "光","Hall of Mirrors","STAGER (ALL STAGE CLEAR)","Linear Accelerator","Tiferet"
);
UPDATE songs SET pack = "Lanota Collaboration" WHERE title IN (
    "Dream goes on","Journey","Specta","Quon","cyanine"
);
UPDATE songs SET pack = "Dynamix Collaboration" WHERE title IN (
    "Moonlight of Sand Castle","REconstruction","Evoltex (poppi'n mix)","Oracle","αterlβus","γuarδina"
);
UPDATE songs SET pack = "?" WHERE title IN (
    "Ignotus Afterburn", "Red and Blue and Green"
);

ALTER TABLE songs MODIFY COLUMN pack VARCHAR(255) NOT NULL;
