use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::sync::RwLock;


#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpRequest {
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

    pub fn req_to_user(&self) -> User {
        User {
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.email.is_empty() || self.password.is_empty() { false } else { true }
    }

    pub fn get_email(&self) -> String {
        self.email.clone()
    }
} 

impl Clone for SignUpRequest {
    fn clone(&self) -> Self {
        SignUpRequest {
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
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

    pub fn req_to_user(&self) -> User {
        User {
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.email.is_empty() || self.password.is_empty() { false } else { true }
    }

}

impl Clone for LoginRequest {
    fn clone(&self) -> Self {
        LoginRequest {
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

#[derive(Debug)]
pub struct User {
    email: String,
    password: String,
}

impl User {
    pub fn new(email: String, password: String) -> Self {
        User {
            email,
            password,
        }
    }
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            email: self.email.clone(),
            password: self.password.clone(),
        }
    }
}

pub struct UserStorage {
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

    pub fn login(&self, user: User) -> Result<User, &'static str> {
        let map = &self.users;
        if let Some(gotuser) = map.get(&(user.email)) {
            if gotuser.password == user.password {
              Ok(gotuser.as_ref().clone())
            } else {
                Err("The password is incorrect")
            }
        } else {
            Err("No account under that email exists")
        }
    }

}

pub struct UsersLock {
    pub userbox: RwLock<UserStorage>,
}

impl UsersLock {
    pub fn new() -> Self {
        UsersLock {
            userbox: RwLock::new(UserStorage::new()),
        }
    }
}
