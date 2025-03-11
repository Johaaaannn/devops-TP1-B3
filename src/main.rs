use actix_web::{get, App, HttpServer, HttpRequest, Responder, HttpResponse};
use std::env;

#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(req.headers().iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<serde_json::Value>())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PING_LISTEN_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    println!("✅ Server starting on http://127.0.0.1:{}", port);

    match HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", port)) {
        Ok(server) => {
            println!("🚀 Server is running on port {}", port);
            server.run().await
        }
        Err(e) => {
            eprintln!("❌ Failed to bind server: {}", e);
            Err(e)
        }
    }
}