use authentication::{LoginAction, LoginRole, greet_user, login, read_line};

fn main() {
    println!("{}", greet_user("LOLA"));
    let mut tries = 0;

    loop {
        println!("Enter your username: ");
        let username = read_line();
        println!("Enter your password: ");
        let password = read_line();

        match login(&username, &password) {
            Some(LoginAction::Granted(LoginRole::Admin)) => {
                println!("Welcome {username}, you are an admin");
                break;
            }
            Some(LoginAction::Granted(LoginRole::User)) => {
                println!("Welcome {username}, you are a user");
                break;
            }
            Some(LoginAction::Denied) => {
                println!("Login failed.");
                tries += 1;
                if tries >= 3 {
                    println!("Too many failed login attempts");
                    break;
                }
            }
            None => {
                println!("New user!");
                break;
            }
        }
    }
}
