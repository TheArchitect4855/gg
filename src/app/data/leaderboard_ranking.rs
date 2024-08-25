use serde::Serialize;

use super::UserId;

#[derive(Serialize)]
pub struct LeaderboardRanking {
	pub rank: usize,
	pub user_id: UserId,
	pub user_name: String,
	pub score: u32,
}
