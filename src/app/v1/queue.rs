use actix_web::{put, web};
use serde::{Deserialize, Serialize};

use crate::app::{
	data::{UserId, UserSecret},
	services::{MatchInfo, MatchMaking, SqliteDatabase},
};

#[derive(Deserialize)]
pub struct QueuePutRequest {
	user_id: UserId,
	user_secret: UserSecret,
}

#[derive(Serialize)]
pub struct QueuePutResponse {
	match_info: MatchInfo,
}

#[put("/queue")]
pub async fn put(
	body: web::Json<QueuePutRequest>,
	db: web::Data<SqliteDatabase>,
	matchmaking: web::Data<MatchMaking>,
) -> actix_web::Result<web::Json<QueuePutResponse>> {
	if !db.is_user_authentic(&body.user_id, &body.user_secret) {
		return Err(actix_web::error::ErrorUnauthorized(
			"invalid user credentials",
		));
	}

	let match_info = matchmaking
		.queue_player(body.user_id.clone(), db.get_ref())
		.await;
	Ok(web::Json(QueuePutResponse { match_info }))
}
