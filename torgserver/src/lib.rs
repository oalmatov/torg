use actix_files as fs;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

mod models;
use models::*;

pub async fn fallback(_req: HttpRequest) -> impl Responder {
    fs::NamedFile::open_async("../dist/index.html").await.unwrap()
}

#[post("/signup")]
pub async fn signup(req: web::Json<SignUpRequest>, data: web::Data<UsersLock>) -> impl Responder {
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


#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_post() {
        let data = web::Data::new( UsersLock::new() );
        let app = test::init_service(App::new()
            .app_data(data.clone())
            .service(signup))
            .await;
        let newUser = SignUpRequest::new("torg@torg.net", "apples");
        let req = test::TestRequest::post()
          .uri("/signup")
          .set_json(newUser)
          .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_signup_userhash() {
        let data = web::Data::new( UsersLock::new() );
        let app = test::init_service(App::new()
            .app_data(data.clone())
            .service(signup))
            .await;
        let newUser = SignUpRequest::new("torg@torg.net", "apples");
        let newUser1 = newUser.req_to_user();
        let req = test::TestRequest::post()
          .uri("/signup")
          .set_json(newUser)
          .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Response Statis: {:?}", resp.status());
        assert!(resp.status().is_success());

        let mut userstor = data.userbox.read().unwrap();

        let result = (*userstor).login(newUser1);
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn test_login_request() {
        let data = web::Data::new( UsersLock::new() );
        let app = test::init_service(App::new()
            .app_data(data.clone())
            .service(login)
            .service(signup))
            .await;
        let newUser = SignUpRequest::new("torg@torg.net", "apples");
        let req = test::TestRequest::post()
          .uri("/signup")
          .set_json(newUser)
          .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let login_user = LoginRequest::new("torg@torg.net", "apples");
        let login_req = test::TestRequest::post()
          .uri("/login")
          .set_json(login_user)
          .to_request();
        let login_resp = test::call_service(&app, login_req).await;

        assert!(login_resp.status().is_success());

        let userReq = LoginRequest::new("torg@torg.net", "wrong_pass");
        let login_req_bad = test::TestRequest::post()
          .uri("/login")
          .set_json(userReq)
          .to_request();
        let login_resp_bad = test::call_service(&app, login_req_bad).await;

        assert!(login_resp_bad.status().is_client_error());


    }

    #[test]
    async fn test_create_user_twice() {
        let mut storage = UserStorage::new();

        let user1 = User::new("geek@geek.com".to_string(), "pass".to_string());
        let mut result = storage.add_user(user1);
        assert!(result.is_ok()); 

        let user2 = User::new("geek@geek.com".to_string(), "wrongpass".to_string());

        result = storage.add_user(user2);
        assert!(result.is_err());
    }

    #[test]
    async fn test_login() {
        let mut storage = UserStorage::new();

        let user1 = User::new("geek@geek.com".to_string(), "pass".to_string()); 
        let result = storage.add_user(user1);

        let user2 = User::new("geek@geek.com".to_string(), "pass".to_string()); 

        let login_result = storage.login(user2);
        assert!(login_result.is_ok());

        let user3 = User::new("geek@geek.com".to_string(), "wrongpass".to_string()); 

        let login_result_fail = storage.login(user3);
        assert!(login_result_fail.is_err());
   }
}
