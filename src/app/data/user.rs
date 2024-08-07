use chrono::{DateTime, Utc};

use super::{UserId, UserSecret};

#[derive(Clone, Debug)]
pub struct User {
	pub id: UserId,
	pub secret: UserSecret,
	pub created_at: DateTime<Utc>,
	pub data: serde_json::Value,
}
