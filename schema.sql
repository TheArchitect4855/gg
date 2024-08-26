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
	('Dirty', 'Dragon'),
	('Esoteric', 'Emu'),
	('Free', 'Fellow'),
	('Ghastly', 'Gunner'),
	('Honorable', 'Hunter'),
	('Irreputable', 'Image'),
	('Jolly', 'Jakalope'),
	('Knowledgable', 'Knut'),
	('Little', 'Lime'),
	('Mystic', 'Melon'),
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

CREATE TABLE IF NOT EXISTS config (
	key TEXT NOT NULL PRIMARY KEY,
	value TEXT
);

INSERT OR IGNORE INTO config (key) VALUES
	('game_version'),
	('s2s_secret')
;

CREATE TABLE IF NOT EXISTS user_scores (
	user_id TEXT NOT NULL REFERENCES users (id),
	score INTEGER NOT NULL,
	created_at TEXT NOT NULL DEFAULT (datetime())
);

CREATE VIEW IF NOT EXISTS leaderboard AS
SELECT user_id, sum(score) AS score
FROM user_scores
GROUP BY user_id;
