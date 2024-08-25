use actix_web::Scope;

mod leaderboard;
mod login;
mod queue;
mod user;

pub fn init(scope: Scope) -> Scope {
	scope
		.service(leaderboard::get)
		.service(leaderboard::put)
		.service(login::login)
		.service(queue::put)
		.service(user::post)
		.service(user::put)
}
