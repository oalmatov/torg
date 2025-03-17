use actix_files as fs;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;

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
        match self.users.insert(user.email.to_string(), Box::new(user)) {
            None => Ok(()),
            Some(_) => Err("User already exists"),
        }
    }

    pub fn login(&mut self, email: String) -> Result<&User, &'static str> {
        if let Some(user) = self.users.get(&email) {
            Ok(user.as_ref())
        } else {
            Err("No account under that email exists")
        }
    }

}

pub async fn fallback(_req: HttpRequest) -> impl Responder {
    fs::NamedFile::open_async("../dist/index.html").await.unwrap()
}

#[post("/signup")]
pub async fn signup(req: web::Json<SignUpRequest>) -> impl Responder {
    if req.email.is_empty() || req.password.is_empty() {
        return HttpResponse::BadRequest().body("All fields must have a value");
    }
    HttpResponse::Created().json(serde_json::json!({
        "message" : "User regustered successfully!",
        "email" :  req.email
    }))
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new()
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
    }

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(App::new()
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
    }

    #[test]
    async fn create_user_twice() {
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

        let login_result = storage.login("geek@geek.com".to_string());
        assert!(result.is_ok());

        let login_result_fail = storage.login("boomer@cabin.com".to_string());
        assert!(login_result_fail.is_err());
   }
}
