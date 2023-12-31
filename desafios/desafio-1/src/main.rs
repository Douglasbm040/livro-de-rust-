use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn saldo() -> impl Responder {
   HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn transacao(req_body: String) -> impl Responder {
   HttpResponse::Ok().body(req_body)
}

async fn init_app() -> impl Responder {
   HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
      App::new()
          .service(saldo)
          .service(transacao)
          .route("/hey", web::get().to(init_app))
   })
       .bind(("127.0.0.1", 8080))?
       .run()
       .await
}

