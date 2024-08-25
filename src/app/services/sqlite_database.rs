use std::path::Path;

use rusqlite::{params, OptionalExtension};

use crate::app::data::{GameMap, UserData, UserId, UserSecret};

pub struct SqliteDatabase {
	connection: rusqlite::Connection,
}

impl SqliteDatabase {
	pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
		let connection = rusqlite::Connection::open(path)?;
		Ok(Self { connection })
	}

	pub fn config_get(&self, key: &str) -> Option<String> {
		self.connection
			.query_row("SELECT value FROM config WHERE key = ?", [key], |row| {
				row.get(0)
			})
			.unwrap()
	}

	pub fn create_user(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
		user_data: &UserData,
	) -> Result<(), rusqlite::Error> {
		let user_data = serde_json::to_value(user_data).unwrap();
		self.connection.execute(
			r#"
			INSERT INTO users (id, secret, data)
			VALUES (?, ?, ?)
		"#,
			params![user_id, user_secret, user_data],
		)?;

		Ok(())
	}

	pub fn get_game_map_random(&self) -> GameMap {
		self.connection
			.query_row(
				r#"
				SELECT name, max_players
				FROM game_maps
				ORDER BY random()
				LIMIT 1
				"#,
				(),
				|row| {
					Ok(GameMap {
						name: row.get(0)?,
						max_players: row.get(1)?,
					})
				},
			)
			.unwrap()
	}

	pub fn get_server_address(&self) -> String {
		if cfg!(debug_assertions) {
			return "localhost".into();
		}

		self.connection
			.query_row(
				"SELECT address FROM server_addresses LIMIT 1",
				(),
				|row| row.get(0),
			)
			.unwrap()
	}

	pub fn get_user_data(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
	) -> Option<UserData> {
		let row: (UserSecret, serde_json::Value) = self
			.connection
			.query_row(
				r#"
			SELECT secret, data
			FROM users
			WHERE id = ?
		"#,
				params![user_id],
				|row| Ok((row.get(0)?, row.get(1)?)),
			)
			.optional()
			.unwrap()?;

		if user_secret == &row.0 {
			Some(serde_json::from_value(row.1).unwrap())
		} else {
			None
		}
	}

	pub fn is_user_authentic(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
	) -> bool {
		let secret: Option<UserSecret> = self
			.connection
			.query_row(
				"SELECT secret FROM users WHERE id = ?",
				params![user_id],
				|row| row.get(0),
			)
			.optional()
			.unwrap();

		let Some(secret) = secret else {
			return false;
		};

		user_secret == &secret
	}

	pub fn update_user_data(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
		user_data: &UserData,
	) {
		let user_data = serde_json::to_value(user_data).unwrap();
		self.connection
			.execute(
				r#"
			UPDATE users
			SET data = ?
			WHERE id = ? AND secret = ?
		"#,
				params![user_data, user_id, user_secret],
			)
			.unwrap();
	}

	pub fn get_user_name_options(&self) -> (Vec<String>, Vec<String>) {
		let mut stmt = self
			.connection
			.prepare(
				r#"
			SELECT adjective, noun
			FROM user_name_generator
		"#,
			)
			.unwrap();

		let mut rows = stmt.query(()).unwrap();
		let mut adjectives = Vec::new();
		let mut nouns = Vec::new();
		while let Some(row) = rows.next().unwrap() {
			let adj: String = row.get(0).unwrap();
			let noun: String = row.get(1).unwrap();
			adjectives.push(adj);
			nouns.push(noun);
		}

		(adjectives, nouns)
	}
}
