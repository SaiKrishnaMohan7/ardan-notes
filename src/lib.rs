#[derive(Debug, PartialEq, Eq)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();

    if username == "admin" && password == "password" {
        LoginAction::Granted(LoginRole::Admin)
    } else if username == "bob" && password == "password" {
        LoginAction::Granted(LoginRole::User)
    } else {
        LoginAction::Denied
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

    fn test_login() {
        assert_eq!(login("admin", "password"), LoginAction::Admin);
        assert_eq!(login("bob", "password"), LoginAction::User);
        assert_eq!(login("admin", "not-password"), LoginAction::Denied);
    }
}
