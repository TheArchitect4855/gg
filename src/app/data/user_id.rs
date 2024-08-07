use rusqlite::{types::FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::base32;

const ID_LEN: usize = 6;

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize)]
pub struct UserId(String);

impl UserId {
	pub async fn new_random() -> Self {
		let id = base32::random_secure(ID_LEN).await;
		Self(id)
	}
}

impl FromSql for UserId {
	fn column_result(
		value: rusqlite::types::ValueRef<'_>,
	) -> rusqlite::types::FromSqlResult<Self> {
		let s = value.as_str()?;
		assert_eq!(s.len(), ID_LEN);
		Ok(Self(s.to_string()))
	}
}

impl ToSql for UserId {
	fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
		Ok(rusqlite::types::ToSqlOutput::Borrowed(
			rusqlite::types::ValueRef::Text(&self.0.as_bytes()),
		))
	}
}
