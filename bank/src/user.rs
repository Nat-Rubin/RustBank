use lazy_static::lazy_static;
use std::io::{stdin, stdout, Write};
use std::sync::Mutex;
use futures::executor::block_on;
use tokio::runtime::Runtime;

use crate::user_dao;

//#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct User {
    pub id: i32,
    pub balance: i32,  // 100 = $1
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

struct ActiveUser {
    active_user: User,
    
}

lazy_static! {
    static ref ACTIVE_USER: Mutex<ActiveUser> = Mutex::new(ActiveUser::default());
}

impl User {
    fn to_string(&self) -> String {
        // declare str: String
        //let mut str: String = "".to_owned();
        
        // turn self.id into str
        let x: i32 = self.id;
        let s: String = x.to_string();
        let ss: &str = &s;
        
        let y: i32 = self.balance;
        let sy: String = y.to_string();
        let ssy: &str = &sy;
        
        // add everything to str and return
        return "id: ".to_string() +
                ss +
                "balance: " +
                ssy +
                " username: " +
                &self.username +
                " password: " +
                &self.password +
                " email: " +
                &self.email +
                " first_name: " +
                &self.first_name +
                " last_name: " +
                &self.last_name;
    }
}

// DAOs
impl User {
    fn add(user: User) {
        println!("Here in user::add()");
        let rt = Runtime::new().unwrap();
        rt.block_on(user_dao::add(user)).unwrap();
    }
}

impl Default for User {
    fn default() -> User {
        User {
            id: 0,
            balance: 0,
            username: String::new(),
            password: String::new(),
            email: String::new(),
            first_name: String::new(),
            last_name: String::new()
        }
    }
}

impl Default for ActiveUser {
    fn default() -> Self {
        ActiveUser { active_user: (User::default()) }
    }
}

pub fn login() {
    let mut input_username = String::new();
    let mut input_password = String::new();
    loop {
        println!("Enter username");
        let _ = stdout().flush();
        stdin().read_line(&mut input_username).unwrap();
        println!("Enter password");
        let _ = stdout().flush();
        stdin().read_line(&mut input_password).unwrap();
        
        if login_if_user_exists(&input_password, &input_password) {
            //println!("ACTIVE_USER: {}",);
            //return;
        }
    }
}

fn login_if_user_exists(username: &str, password: &str) -> bool {
    // check database and stuff

    let current_user = User {
        id: get_new_id(),
        balance: 0,
        username: String::from("Username"),
        password: String::from("Password"),
        email: String::from("Email"),
        first_name: String::from("Firstname"),
        last_name: String::from("Lastname"),
    };

    println!("Active User: {}", current_user.to_string().clone());
    ACTIVE_USER.lock().unwrap().active_user = current_user;
    
    return true
}

pub fn create_new_account() {
    let mut username = String::new();
    let mut password = String::new();
    let mut email = String::new();
    let mut first_name = String::new();
    let mut last_name = String::new();
    
    println!("Please enter a username");
    let _ = stdout().flush();
    stdin().read_line(&mut username).unwrap();
    
    println!("Please enter a password");
    let _ = stdout().flush();
    stdin().read_line(&mut password).unwrap();
    
    println!("Please enter an email");
    let _ = stdout().flush();
    stdin().read_line(&mut email).unwrap();
    
    println!("Please enter your first name");
    let _ = stdout().flush();
    stdin().read_line(&mut first_name).unwrap();
    
    println!("Please enter you last name");
    let _ = stdout().flush();
    stdin().read_line(&mut last_name).unwrap();
    
    User::add(build_new_user(username, password, email, first_name, last_name))
}

fn build_new_user(
    username: String,
    password: String,
    email: String,
    first_name: String,
    last_name: String,
) -> User {
    User {
        id: get_new_id(),
        balance: 0,
        username: username,
        password: password,
        email: email,
        first_name: first_name,
        last_name: last_name,
    }
}

fn get_new_id() -> i32 {
    return 1;
}
