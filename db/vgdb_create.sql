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

  FOREIGN KEY(platform_id) REFERENCES platforms(id)
);
-- SCHEMA END

-------------------
-- DATA START
-------------------
INSERT INTO platforms (id, release_date, title, logo_url) VALUES
  (1, "1983-07-15", "NES", "/platform/nes.png"),
  (2, "1988-10-29", "Genesis", "/platform/genesis.png"),
  (3, "1990-11-21", "SNES", "/platform/snes.svg"),
  (4, "1994-11-22", "Saturn", "/platform/saturn.png"),
  (5, "1994-12-03", "PSX", "/platform/psx.png"),
  (6, "1996-06-23", "N64", "/platform/n64.png"),
  (7, "1998-11-27", "Dreamcast", "/platform/dreamcast.png"),
  (8, "2000-03-04", "PS2", "/platform/ps2.png"),
  (9, "2006-11-11", "PS3", "/platform/ps3.png");

-- SNES: 3
INSERT INTO games (platform_id, release_date, title, boxart_url) VALUES
  (3, "1990-11-21", "Super Mario World", "/title/super-mario-world.png"),
  (3, "1991-11-21", "The Legend of Zelda: A Link to the Past", "/title/zelda-link-to-the-past.png"),
  (3, "1994-03-19", "Super Metroid", "/title/super-metroid.png"),
  (3, "1994-08-27", "Earthbound", "/title/earthbound.png");
-- DATA END
