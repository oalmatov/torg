use torgserver::*;
use actix_files as fs;
use actix_web::{App, HttpServer, web};


#[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(fs::Files::new("/", "../dist").index_file("index.html"))
                .default_service(web::to(fallback))
                .service(signup)
        }) 
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
