use actix_web::{middleware::Logger, web, App, HttpServer};
use gg::app::services::SqliteDatabase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	init_logging();

	let addr = if cfg!(debug_assertions) {
		("127.0.0.1", 8080)
	} else {
		("0.0.0.0", 80)
	};

	HttpServer::new(|| {
		// Open a database connection per-thread. This should help prevent
		// issues w.r.t. locking and transactions (maybe).
		let database = SqliteDatabase::open("database.sqlite")
			.expect("failed to open database");

		let v1 = gg::app::v1::init(web::scope("/v1"));
		App::new()
			.wrap(Logger::default())
			.app_data(web::Data::new(database))
			.service(v1)
	})
	.bind(addr)?
	.run()
	.await
}

fn init_logging() {
	let env = env_logger::Env::new().filter_or("LOG_LEVEL", "info");
	env_logger::init_from_env(env);
}
