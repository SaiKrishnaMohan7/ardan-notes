use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::Path,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

pub fn get_users() -> Vec<User> {
    let admin_user = User::new("admin", "password", LoginRole::Admin);
    let reg_user = User::new("bob", "password", LoginRole::User);
    let users = vec![admin_user, reg_user];

    users
}

pub fn get_users_map() -> HashMap<String, User> {
    let admin_user = User::new("admin", "password", LoginRole::Admin);
    let reg_user = User::new("bob", "password", LoginRole::User);
    let mut users = HashMap::new();

    users.insert("admin".to_string(), admin_user);
    users.insert("bob".to_string(), reg_user);

    users
}

pub fn save_users(users: HashMap<String, User>) {
    let users_path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();

    fs::write(users_path, users_json).unwrap();
}

pub fn delete_user(username: String) {
    let users_path = Path::new("users.json");
    let users_json = read_to_string(users_path).unwrap();
    let mut users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();

    users.remove(&username).unwrap();
    save_users(users);
}

pub fn get_users_or_create_json() -> HashMap<String, User> {
    let users_path = Path::new("users.json");

    if users_path.exists() {
        let users_json = fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();

        users
    } else {
        let users = get_users_map();
        let users_json = serde_json::to_string(&users).unwrap();
        fs::write(users_path, users_json).unwrap();

        users
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02X}")) // each byte → 2 hex digits; 02 keeps the leading zero
        .collect() // into String: the return type is the destination
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let users = get_users();
    #[allow(unused)]
    let user_map = get_users_or_create_json();
    let username = username.to_lowercase();
    let password = hash_password(password);

    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    } else {
        return None;
    }

    #[allow(unused)]
    if let Some(user) = user_map.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    } else {
        return None;
    }
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Stdin not working");

    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = greet_user("Tim");
        assert_eq!(result, "Hello Tim".to_string());
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
    }
}
