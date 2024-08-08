use actix_web::{post, web};
use serde::Deserialize;

use crate::app::{
	data::{UserId, UserSecret},
	v1::{database::SqliteDatabase, user::UserData},
};

#[derive(Deserialize)]
pub struct LoginRequest {
	user_id: UserId,
	user_secret: UserSecret,
}

#[post("/login")]
pub async fn login(
	body: web::Json<LoginRequest>,
	db: web::Data<SqliteDatabase>,
) -> Result<web::Json<UserData>, actix_web::Error> {
	let Some(user_data) = db.get_user_data(&body.user_id, &body.user_secret)
	else {
		return Err(actix_web::error::ErrorNotFound("invalid login info"));
	};

	Ok(web::Json(user_data))
}
