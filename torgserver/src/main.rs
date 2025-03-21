use torgserver::*;
use actix_files as fs;
use actix_web::{App, HttpServer, web};

mod models;

use models::*;

#[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let data = web::Data::new( UsersLock::new() );
        HttpServer::new(move || {
            App::new()
                .service(fs::Files::new("/", "../dist").index_file("index.html"))
                .default_service(web::to(fallback))
                .app_data(data.clone())
                .service(signup)
                .service(login)
        }) 
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
