use actix_web::{get, put, web};
use serde::{Deserialize, Serialize};

use crate::app::{
	data::{LeaderboardRanking, UserId},
	services::SqliteDatabase,
};

#[derive(Deserialize)]
pub struct GetLeaderboardRequest {
	user_id: Option<UserId>,
}

#[derive(Serialize)]
pub struct GetLeaderboardResponse {
	rankings: Vec<LeaderboardRanking>,
}

#[derive(Deserialize)]
pub struct PutLeaderboardRequest {
	secret: String,
	score: u32,
	user_id: UserId,
}

#[get("/leaderboard")]
pub async fn get(
	req: web::Query<GetLeaderboardRequest>,
	db: web::Data<SqliteDatabase>,
) -> Result<web::Json<GetLeaderboardResponse>, actix_web::Error> {
	let rankings = db
		.get_leaderboard_rankings(9, req.user_id.clone())
		.map_err(|e| actix_web::error::ErrorUnprocessableEntity(e))?;
	Ok(web::Json(GetLeaderboardResponse { rankings }))
}

#[put("/leaderboard")]
pub async fn put(
	req: web::Json<PutLeaderboardRequest>,
	db: web::Data<SqliteDatabase>,
) -> actix_web::HttpResponse {
	let secret = db.config_get("s2s_secret").expect("missing s2s_secret");
	if req.secret != secret {
		return actix_web::HttpResponse::from_error(
			actix_web::error::ErrorForbidden("incorrect secret"),
		);
	}

	if let Err(e) = db.set_leaderboard_score(req.user_id.clone(), req.score) {
		return actix_web::HttpResponse::from_error(
			actix_web::error::ErrorUnprocessableEntity(e),
		);
	}

	actix_web::HttpResponse::Ok().finish()
}
