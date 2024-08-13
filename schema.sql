CREATE TABLE IF NOT EXISTS users (
	id TEXT NOT NULL PRIMARY KEY,
	secret TEXT NOT NULL UNIQUE,
	data JSONB NOT NULL,
	created_at TEXT NOT NULL DEFAULT (datetime())
);

CREATE TABLE IF NOT EXISTS user_name_generator (
	adjective TEXT NOT NULL UNIQUE,
	noun TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO user_name_generator (adjective, noun)
VALUES ('Alluring', 'Apple'),
	('Bright', 'Banjo'),
	('Crusty', 'Comrade'),
	('Dirty', 'Daddy'),
	('Esoteric', 'Emu'),
	('Free', 'Fellow'),
	('Ghastly', 'Gunner'),
	('Honorable', 'Hunter'),
	('Irreputable', 'Image'),
	('Jolly', 'Jakalope'),
	('Knowledgable', 'Knut'),
	('Little', 'Lime'),
	('Mystic', 'Mommy'),
	('Notorious', 'Nob'),
	('Octarine', 'Octopus'),
	('Prickly', 'Pear'),
	('Quiet', 'Qiana'),
	('Reputable', 'Racketeer'),
	('Salacious', 'Singer'),
	('Tough', 'Trombone'),
	('Unreal', 'Underdog'),
	('Veracious', 'Vinegar'),
	('Wonky', 'Wheels'),
	('Xylographic', 'Xylophone'),
	('Yelling', 'Yankee'),
	('Zoinked', 'Zoomer')
;

CREATE TABLE IF NOT EXISTS server_addresses (
	address TEXT NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS game_maps (
	name TEXT NOT NULL UNIQUE,
	max_players INTEGER NOT NULL
);

INSERT OR IGNORE INTO game_maps (name, max_players)
VALUES ('dm_testbed', 4);
