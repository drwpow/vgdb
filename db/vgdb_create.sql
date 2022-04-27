-------------------
-- SCHEMA START
-------------------
CREATE TABLE platforms (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title TEXT NOT NULL,
  release_date TEXT NOT NULL,
  logo_url TEXT NOT NULL
);

CREATE TABLE games (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  platform_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  release_date TEXT NOT NULL,
  boxart_url TEXT,
  description TEXT,
  metacritic_score REAL,
  game_ranking_score REAL,

  FOREIGN KEY(platform_id) REFERENCES platforms(id)
);
-- SCHEMA END

-------------------
-- DATA START
-------------------
INSERT INTO platforms (id, release_date, title, logo_url) VALUES
  (1, "1983-07-15", "NES", "/assets/platform/nes.png"),
  (2, "1988-10-29", "Genesis", "/assets/platform/genesis.png"),
  (3, "1990-11-21", "SNES", "/assets/platform/snes.svg"),
  (4, "1994-11-22", "Saturn", "/assets/platform/saturn.png"),
  (5, "1994-12-03", "PSX", "/assets/platform/psx.png"),
  (6, "1996-06-23", "N64", "/assets/platform/n64.png"),
  (7, "1998-11-27", "Dreamcast", "/assets/platform/dreamcast.png"),
  (8, "2000-03-04", "PS2", "/assets/platform/ps2.png"),
  (9, "2006-11-11", "PS3", "/assets/platform/ps3.png");

-- SNES: 3
INSERT INTO games (platform_id, release_date, title, boxart_url, game_ranking_score) VALUES
  (3, "1990-11-21", "Super Mario World", "/assets/snes-usa/Super Mario World (USA).png", 94.44),
  (3, "1991-10-31", "Super Castlevania IV", "/assets/snes-usa/Super Castlevania IV (USA).png", 81.21),
  (3, "1991-11-21", "The Legend of Zelda: A Link to the Past", "/assets/snes-usa/Legend of Zelda, The - A Link to the Past (USA).png", 92.87),
  (3, "1993-08-06", "Secret of Mana", "/assets/snes-usa/Secret of Mana (USA).png", 87.28),
  (3, "1993-12-17", "Mega Man X", "/assets/snes-usa/Mega Man X (USA).png", 87.27),
  (3, "1994-03-19", "Super Metroid", "/assets/snes-usa/Super Metroid (USA).png", 96.55),
  (3, "1994-04-02", "Final Fantasy III", "/assets/snes-usa/Final Fantasy III (USA).png", 93.96),
  (3, "1994-08-27", "EarthBound", "/assets/snes-usa/EarthBound (USA).png", 88.33),
  (3, "1994-11-18", "Donkey Kong Country", "/assets/snes-usa/Donkey Kong Country (USA).png", 87.81),
  (3, "1995-03-11", "Chrono Trigger", "/assets/snes-usa/Chrono Trigger (USA).png", 94.88),
  (3, "1996-03-09", "Super Mario RPG: Legend of the Seven Stars", "/assets/snes-usa/Super Mario RPG - Legend of the Seven Stars (USA).png", 89.12);
-- DATA END
