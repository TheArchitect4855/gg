use actix_web::{middleware::Logger, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	init_logging();

	let addr = if cfg!(debug_assertions) {
		("127.0.0.1", 8080)
	} else {
		("0.0.0.0", 80)
	};

	HttpServer::new(|| {
		let v1 = gg::app::v1::init(web::scope("/v1"));
		App::new().wrap(Logger::default()).service(v1)
	})
	.bind(addr)?
	.run()
	.await
}

fn init_logging() {
	let env = env_logger::Env::new().filter_or("LOG_LEVEL", "info");
	env_logger::init_from_env(env);
}
