use actix_web::{web, Scope};
use database::SqliteDatabase;

mod database;
mod login;
mod user;

pub fn init(scope: Scope) -> Scope {
	let database = SqliteDatabase::open("database.sqlite")
		.expect("failed to open database");
	let database = web::Data::new(database);
	scope
		.app_data(database.clone())
		.service(login::login)
		.service(user::post)
		.service(user::put)
}
