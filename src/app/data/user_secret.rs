use std::fmt::Debug;

use rusqlite::{types::FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::rng;

const SECRET_LEN: usize = 32;

#[derive(Clone, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub struct UserSecret(String);

impl UserSecret {
	pub async fn new_random() -> Self {
		let secret = rng::gen_base32_secure(SECRET_LEN).await;
		Self(secret)
	}
}

impl Debug for UserSecret {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "UserSecret")
	}
}

impl FromSql for UserSecret {
	fn column_result(
		value: rusqlite::types::ValueRef<'_>,
	) -> rusqlite::types::FromSqlResult<Self> {
		let s = value.as_str()?;
		assert_eq!(s.len(), SECRET_LEN);
		Ok(Self(s.to_string()))
	}
}

impl ToSql for UserSecret {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok(rusqlite::types::ToSqlOutput::Borrowed(
			rusqlite::types::ValueRef::Text(&self.0.as_bytes()),
		))
	}
}
