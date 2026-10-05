use std::{
    io,
    sync::{LazyLock, RwLock},
    thread, time,
};

// first time this gets created, it will be empty
// first time anything looks at it build_users will fire
static USERS: LazyLock<RwLock<Vec<String>>> = LazyLock::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
fn main() {
    thread::spawn(|| {
        loop {
            println!("Current users (in a thread)");
            let users = USERS.read().unwrap();
            println!("{users:?}");
            thread::sleep(time::Duration::from_secs(3));
        }
    });

    loop {
        println!("Enter a name to add to the user list or q to quit");
        let input = read_line();
        if input == "q" {
            break;
        } else {
            let mut lock = USERS.write().unwrap();
            lock.push(input);
        }
    }
}
