use std::path::Path;

use rusqlite::{params, OptionalExtension};

use crate::app::data::{UserId, UserSecret};

pub struct SqliteDatabase {
	connection: rusqlite::Connection,
}

impl SqliteDatabase {
	pub fn open(path: impl AsRef<Path>) -> Result<Self, rusqlite::Error> {
		let connection = rusqlite::Connection::open(path)?;
		Ok(Self { connection })
	}

	pub fn create_user(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
		user_data: &serde_json::Value,
	) -> Result<(), rusqlite::Error> {
		self.connection.execute(
			r#"
			INSERT INTO users (id, secret, data)
			VALUES (?, ?, ?)
		"#,
			params![user_id, user_secret, user_data],
		)?;

		Ok(())
	}

	pub fn get_user_data(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
	) -> Option<serde_json::Value> {
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
			Some(row.1)
		} else {
			None
		}
	}

	pub fn update_user_data(
		&self,
		user_id: &UserId,
		user_secret: &UserSecret,
		user_data: &serde_json::Value,
	) {
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
}
