use actix_web::{post, put, web};
use serde::{Deserialize, Serialize};

use crate::app::{
	data::{UserId, UserSecret},
	v1::database::SqliteDatabase,
};

#[derive(Deserialize, Serialize)]
pub struct UserCreateRequest {
	user_name: String,
}

#[derive(Serialize)]
pub struct UserCreateResponse {
	user_id: UserId,
	user_secret: UserSecret,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
	user_id: UserId,
	user_secret: UserSecret,
	user_data: UserCreateRequest,
}

#[post("/user")]
pub async fn post(
	body: web::Json<UserCreateRequest>,
	db: web::Data<SqliteDatabase>,
) -> Result<web::Json<UserCreateResponse>, actix_web::Error> {
	let user_id = UserId::new_random().await;
	let user_secret = UserSecret::new_random().await;
	let user_data = serde_json::to_value(body.0).unwrap();
	db.create_user(&user_id, &user_secret, &user_data)
		.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
	Ok(web::Json(UserCreateResponse {
		user_id,
		user_secret,
	}))
}

#[put("/user")]
pub async fn put(
	body: web::Json<UserUpdateRequest>,
	db: web::Data<SqliteDatabase>,
) -> actix_web::HttpResponse {
	let user_data = serde_json::to_value(&body.0.user_data).unwrap();
	db.update_user_data(&body.user_id, &body.user_secret, &user_data);
	actix_web::HttpResponse::Ok().finish()
}
