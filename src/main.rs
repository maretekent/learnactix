use actix_web::{ web, App, HttpResponse, HttpServer, Responder};


async fn index () -> impl Responder {
	println!("executing the index method...");
	HttpResponse::Ok().body("Hello world")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting web application");
	HttpServer::new( || {
		App::new()
			.service(
				web::scope("/app")
					.route("/index.html", web::get().to(index))
			)
	})
		.bind("0.0.0.0:5001")?
		.run()
		.await
}
