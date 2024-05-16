use std::{collections::HashMap, path::Path};
use serde::{Serialize, Deserialize};

pub fn hash_password(password: &str)-> String{
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub fn greet_name(name: &str) -> String {
    format!("Hello {name}")
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

fn get_default_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("bob".to_string(), User::new("bob", "password", LoginRole::User));
    users
}

pub fn save_users(users: HashMap<String, User>){
    let users_path = Path::new("../users.json");
    let users_json = serde_json::to_string_pretty(&users).expect("Failed to serialize users");
    std::fs::write(users_path, users_json).expect("Failed to write users.json")

}

pub fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("../users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).expect("Failed to read users.json");
        serde_json::from_str(&users_json).expect("Failed to parse users.json")
    } else {
        let users = get_default_users();
        let users_json = serde_json::to_string_pretty(&users).expect("Failed to serialize users");
        std::fs::write(users_path, users_json).expect("Failed to write users.json");
        users
    }
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();
    let password = hash_password(password);

    if let Some(user) = users.get(&username) {
        if user.password == password {
            Some(LoginAction::Granted(user.role.clone()))
        } else {
            Some(LoginAction::Denied)
        }
    } else {
        None
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Pranav", greet_name("Pranav"));
    }

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), Some(LoginAction::Granted(LoginRole::Admin)));
        assert_eq!(login("Bob", "password"), Some(LoginAction::Granted(LoginRole::User)));
        assert_eq!(login("sassa", "pasjhsword"), Some(LoginAction::Denied));
    }
}
