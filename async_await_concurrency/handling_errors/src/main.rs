use core::error;
use std::{fs, io, path::Path};

use serde::Deserialize;

// anyhow and thiserror are common packages for handling errors

fn maybe_read_file() -> Result<String, io::Error> {
    let my_file = Path::new("my_file.txt");
    fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, io::Error> {
    let contents = maybe_read_file()?;
    Ok(contents.to_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

// Box, a pointer to an area on the heap
// dyn, dynamic, we don't know at compile time what the error is going to be,
// so this will figured out during run time.
type GenericResult<T> = Result<T, Box<dyn error::Error>>;

fn load_users() -> GenericResult<Vec<User>> {
    let my_path = Path::new("users.json");
    let raw_text = fs::read_to_string(my_path)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}
fn main() {
    if let Ok(content) = file_to_uppercase() {
        println!("Contents: {content}");
    }
    let my_file = Path::new("my_file.txt");
    let file_content = fs::read_to_string(my_file);

    match file_content {
        Ok(contents) => println!("{contents}"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found - my_file.txt"),
            _ => println!("Error: {e:#?}"),
        },
    }
}
