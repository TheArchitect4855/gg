use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let addr = if cfg!(debug_assertions) {
		("127.0.0.1", 8080)
	} else {
		("0.0.0.0", 80)
	};

	HttpServer::new(|| {
		let v1 = gg::app::v1::init(web::scope("/v1"));
		App::new().service(v1)
	})
	.bind(addr)?
	.run()
	.await
}
