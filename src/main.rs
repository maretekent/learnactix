use actix_web::{ web, App, HttpResponse, HttpServer, Responder, middleware::Logger};

struct AppState {
	app_name: String,
}

async fn index () -> impl Responder {
	log::info!("executing the index method...");
	HttpResponse::Ok().body("Hello world")
}

#[get("/state")]
async fn testState (data: web::Data<AppState>) -> String {
	let app_name = &data.app_name;
	format!("Hello {:?}", app_name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "info");
	env_logger::init();
    log::info!("starting web application...");
	HttpServer::new( move || {
		App::new()
			.wrap(Logger::default())
			.app_data(
				web::Data::new(AppState {
					app_name: String::from("Actix Web"),
				})
			)
			// .service(
			// 	web::scope("/app")
			// 		.route("/index.html", web::get().to(index))
			// )
			.service(testState)
	})
		.bind("0.0.0.0:5001")?
		.run()
		.await
}
