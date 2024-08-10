use actix_web::Scope;

mod login;
mod user;

pub fn init(scope: Scope) -> Scope {
	scope
		.service(login::login)
		.service(user::post)
		.service(user::put)
}
