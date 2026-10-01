use authentication::{greet_user, login, read_line};

fn main() {
    println!("{}", greet_user("LOLA"));
    let mut tries = 0;

    loop {
        println!("Enter your username: ");
        let username = read_line();
        println!("Enter your password: ");
        let password = read_line();

        if login(&username, &password) {
            println!("Welcome");
            break;
        } else {
            println!("Incorrect username or password");
            tries += 1;
            if tries >= 3 {
                println!("Too many failed login attempts");
                break;
            }
        }
    }
}
