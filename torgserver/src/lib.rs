use actix_files as fs;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Serialize, Deserialize)]
struct SignUpRequest {
    email: String,
    password: String,
}

impl SignUpRequest {
    pub fn new(email: &str, password: &str) -> Self {
        SignUpRequest {
            email: String::from(email),
            password: String::from(password),
        }
    }

    pub fn conv_user(self) -> User {
        User {
            email: self.email,
            password: self.password,
        }
    }
} 

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

impl LoginRequest {
    pub fn new(email: &str, password: &str) -> Self {
        LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Debug)]
struct User {
    email: String,
    password: String,
}

struct UserStorage {
    users: HashMap<String, Box<User>>,
}

impl UserStorage {
    pub fn new() -> UserStorage {
        let map: HashMap<String, Box<User>> = HashMap::new();
        UserStorage {
            users: map,
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), &'static str> {
        let map = &mut self.users; 
        match map.insert(user.email.to_string(), Box::new(user)) {
            None => Ok(()),
            Some(_) => Err("User already exists"),
        }
    }

    pub fn login(&self, email: String, password: String) -> Result<User, &'static str> {
        let map = &self.users;
        if let Some(user) = map.get(&email) {
            let retuser = User {
                email: user.email.clone(),
                password: user.password.clone(),
            };
            if retuser.password == password {
              Ok(retuser)
            } else {
                Err("The password is incorrect")
            }
        } else {
            Err("No account under that email exists")
        }
    }

}

pub async fn fallback(_req: HttpRequest) -> impl Responder {
    fs::NamedFile::open_async("../dist/index.html").await.unwrap()
}

#[post("/signup")]
pub async fn signup(req: web::Json<SignUpRequest>, data: web::Data<UsersLock>) -> impl Responder {
    if req.email.is_empty() || req.password.is_empty() {
        return HttpResponse::BadRequest().body("All fields must have a value");
    }
    let mut users = data.userbox.write().unwrap();
    (*users).add_user(User {email: req.email.clone(), password: req.password.clone()});
    HttpResponse::Created().json(serde_json::json!({
        "message" : "User registered successfully!",
        "email" :  req.email
    }))
}

//need to validate login and give session token back to client
#[post("/login")]
pub async fn login(req: web::Json<LoginRequest>, data: web::Data::<UsersLock>) -> impl Responder {
    if req.email.is_empty() || req.password.is_empty() {
        return HttpResponse::BadRequest().body("All fields must have a value");
    }
    let users = data.userbox.read().unwrap();
    if let Ok(login_result) = (*users).login(req.email.clone(), req.password.clone()) {
        return HttpResponse::Accepted().body("Login success");
    } else {
        return HttpResponse::Forbidden().body("Incorrect Password");
    }

}

struct UsersLock {
    userbox: RwLock<UserStorage>,
}


#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_post() {
        let data = web::Data::new( UsersLock { userbox: RwLock::new(UserStorage::new()) });
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
        let data = web::Data::new( UsersLock { userbox: RwLock::new(UserStorage::new()) });
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
        println!("Response Statis: {:?}", resp.status());
        assert!(resp.status().is_success());

        let mut userstor = data.userbox.read().unwrap();

        let result = (*userstor).login("torg@torg.net".to_string(), "apples".to_string());
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn test_login_request() {
        let data = web::Data::new( UsersLock { userbox: RwLock::new(UserStorage::new()) });
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

        let user1 = User {
            email: "geek@geek.com".to_string(),
            password: "pass".to_string(),
        };

        let mut result = storage.add_user(user1);
        assert!(result.is_ok()); 

        let user2 = User {
            email: "geek@geek.com".to_string(),
            password: "pass".to_string(),
        };

        result = storage.add_user(user2);
        assert!(result.is_err());
    }

    #[test]
    async fn test_login() {
        let mut storage = UserStorage::new();

        let user1 = User {
            email: "geek@geek.com".to_string(),
            password: "pass".to_string(),
        };

        let result = storage.add_user(user1);

        let login_result = storage.login("geek@geek.com".to_string(), "pass".to_string());
        assert!(login_result.is_ok());

        let login_result_fail = storage.login("boomer@cabin.com".to_string(), "weird pass".to_string());
        assert!(login_result_fail.is_err());
   }
}
