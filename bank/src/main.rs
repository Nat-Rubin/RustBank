use std::io::{stdin,stdout,Write};
use std::error::Error;
mod user;
mod globals;
mod user_dao;


// Bank!
// login, logout, new user, change password, email, hash password
// Account: id, username, first name, last name, balance
// withdraw, deposit

fn main() -> Result<(), Box<dyn Error>>{
    user_dao::db_init().unwrap();
    
    let mut user_input;
    loop {
        user_input = String::new();
        println!("Welcome! What would you like to do?");
        let _=stdout().flush();
        stdin().read_line(&mut user_input).unwrap();
        user_input = user_input.to_lowercase();
        user_input.truncate(user_input.len()-1);
        
        println!("{user_input}");
        match user_input.as_str() {
            "login" => user::login(),
            "create new account" => user::create_new_account(),
            _ => println!("Please input a valid option"),
        }
    }
}