use actix_files as fs;
use actix_web::{App, HttpServer, HttpRequest, HttpResponse, Responder, web};

async fn fallback(_req: HttpRequest) -> impl Responder {
    actix_files::NamedFile::open_async("../dist/index.html").await.unwrap()
}

#[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
        // move counter into the closure
        App::new()
            .service(fs::Files::new("/", "../dist").index_file("index.html"))
            .default_service(web::to(fallback))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
