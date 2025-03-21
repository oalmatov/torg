use torgserver::*;
use actix_files as fs;
use actix_web::{App, HttpServer, Responder, HttpRequest, HttpResponse, web, post, http::header};
use actix_cors::Cors;

mod models;

use models::*;

#[post("/signup")]
pub async fn signup(req: web::Json<SignUpRequest>, data: web::Data<UsersLock>) -> impl Responder {
    println!("Received signup: {:?}", req);
    if !req.is_valid() {
        return HttpResponse::BadRequest().body("All fields must have a value");
    }
    let mut users = data.userbox.write().unwrap();
    let email = req.get_email();
    (*users).add_user(req.req_to_user());
    HttpResponse::Created().json(serde_json::json!({
        "message" : "User registered successfully!",
        "email" :  email,
    }))
}

//need to validate login and give session token back to client
#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>, data: web::Data::<UsersLock>) -> impl Responder {
    if !req.is_valid() {
        return HttpResponse::BadRequest().body("All fields must have a value");
    }
    let users = data.userbox.read().unwrap();
    if let Ok(login_result) = (*users).login(req.req_to_user()) {
        return HttpResponse::Accepted().body("Login success");
    } else {
        return HttpResponse::Forbidden().body("Incorrect Password");
    }

}

#[actix_web::main]
    async fn main() -> std::io::Result<()> {
        let data = web::Data::new( UsersLock::new() );
        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_origin("http://localhost:5173")
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_headers(vec![header::CONTENT_TYPE])
                        .max_age(3600),
                )
                .app_data(data.clone())
                .service(signup)
                .service(login)
                .service(fs::Files::new("/", "../dist").index_file("index.html"))
                .default_service(web::to(fallback))
        }) 
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
