use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
	println!("calling hello ...");
	HttpResponse::Ok().body("hello world!")
}

#[post("/echo")]
async fn echo (req: String) -> impl Responder {
	println!("calling echo ....");
	HttpResponse::Ok().body(req)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting web application");
	HttpServer::new( || {
		App::new()
			.service(hello)
			.service(echo)
	})
		.bind("0.0.0.0:5001")?
		.run()
		.await
}
