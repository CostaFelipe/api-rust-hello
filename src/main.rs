use actix_web:: {App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    // add code here
    HttpResponse::Ok().body("Hello world")
} 

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
        .bind("127.0.0.1:5000")?
        .run()
        .await
}