use actix_web::{ web, App, HttpResponse, HttpServer, Responder, middleware::Logger};


async fn index () -> impl Responder {
	log::info!("executing the index method...");
	HttpResponse::Ok().body("Hello world")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "info");
	env_logger::init();
    log::info!("starting web application...");
	HttpServer::new( move || {
		App::new()
			.wrap(Logger::default())
			.service(
				web::scope("/app")
					.route("/index.html", web::get().to(index))
			)
	})
		.bind("0.0.0.0:5001")?
		.run()
		.await
}
