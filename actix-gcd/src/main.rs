use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().route("/", web::get().to(get_index)));

    println!("Serving on http://localhost:8080");

    server
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn get_index() -> String {
    String::from("hello world")
}
