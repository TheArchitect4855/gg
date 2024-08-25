use actix_web::{post, web};
use serde::{Deserialize, Serialize};

use crate::app::{
	data::{UserData, UserId, UserSecret},
	services::SqliteDatabase,
};

#[derive(Deserialize)]
pub struct LoginRequest {
	user_id: UserId,
	user_secret: UserSecret,
}

#[derive(Serialize)]
pub struct LoginResponse {
	game_version: Option<String>,
	user_data: UserData,
}

#[post("/login")]
pub async fn login(
	body: web::Json<LoginRequest>,
	db: web::Data<SqliteDatabase>,
) -> Result<web::Json<LoginResponse>, actix_web::Error> {
	let Some(user_data) = db.get_user_data(&body.user_id, &body.user_secret)
	else {
		return Err(actix_web::error::ErrorNotFound("invalid login info"));
	};

	let game_version = db.config_get("game_version");
	Ok(web::Json(LoginResponse {
		game_version,
		user_data,
	}))
}
