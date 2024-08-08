use actix_web::{post, put, web};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::app::{
	data::{UserId, UserSecret},
	v1::database::SqliteDatabase,
};

#[derive(Serialize)]
pub struct UserCreateResponse {
	user_id: UserId,
	user_secret: UserSecret,
	user_data: UserData,
}

#[derive(Deserialize)]
pub struct UserUpdateRequest {
	user_id: UserId,
	user_secret: UserSecret,
	user_data: UserData,
}

#[derive(Deserialize, Serialize)]
pub struct UserData {
	name: String,
}

#[post("/user")]
pub async fn post(
	db: web::Data<SqliteDatabase>,
) -> Result<web::Json<UserCreateResponse>, actix_web::Error> {
	let user_id = UserId::new_random().await;
	let user_secret = UserSecret::new_random().await;
	let user_data = UserData {
		name: generate_random_username(db.get_ref()),
	};

	db.create_user(&user_id, &user_secret, &user_data)
		.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
	Ok(web::Json(UserCreateResponse {
		user_id,
		user_secret,
		user_data,
	}))
}

#[put("/user")]
pub async fn put(
	body: web::Json<UserUpdateRequest>,
	db: web::Data<SqliteDatabase>,
) -> actix_web::HttpResponse {
	db.update_user_data(&body.user_id, &body.user_secret, &body.0.user_data);
	actix_web::HttpResponse::Ok().finish()
}

fn generate_random_username(db: &SqliteDatabase) -> String {
	let mut rng = rand::thread_rng();
	let (adjectives, nouns) = db.get_user_name_options();
	let i = rng.gen_range(0..adjectives.len());
	let j = rng.gen_range(0..nouns.len());
	format!("{} {}", adjectives[i], nouns[j])
}
