use actix_web::Scope;

mod login;
mod queue;
mod user;

pub fn init(scope: Scope) -> Scope {
	scope
		.service(login::login)
		.service(queue::put)
		.service(user::post)
		.service(user::put)
}
