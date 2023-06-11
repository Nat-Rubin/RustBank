use std::error::Error;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::{FromRow, Row};
use sqlx::pool::PoolOptions;
use sqlx::Postgres;

use crate::user::User;
use crate::globals::DAO_INFO;


fn connect() {
    
}

async fn create_tables() -> Result<(), Box<dyn Error>>{
    println!("Here in create_tables1");
    let url = "postgres://postgres:password@localhost:5432/postgres";
    let pool = PgPoolOptions::new()
    .connect(url)
    .await?;
    
    sqlx::query(
        r#"CREATE SCHEMA IF NOT EXISTS bank"#
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bank.users (
            ID int UNIQUE PRIMARY KEY,
            Balance int NOT NULL,
            Username varchar UNIQUE NOT NULL,
            Password varchar NOT NULL,
            Email varchar NOT NULL,
            Firstname varchar NOT NULL,
            Lastname varchar NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await?;

    println!("Users table made");
    
    return Ok(())
}

#[tokio::main]
pub async fn db_init() -> Result<(), Box<dyn Error>>{
    //connect();
    create_tables().await?;
    
    Ok(())
}

pub async fn add(user: User) -> Result<(), Box<dyn Error>>{
    println!("Here in user_dao::add()");
    let url = "postgres://postgres:password@192.168.1.223:5432/postgres";
    let pool = PgPoolOptions::new()
    .connect(url)
    .await?;
    
    let query = "INSERT INTO ".to_string() + "bank.users" +
                " (ID, balance, username, password, email, firstName, lastName) " +
                "VALUES ($1, $2, $3, $4, $5, $6, $7)";
    
    let query_slice = &query[..];
    
    sqlx::query(query_slice)
        .bind(&user.id)
        .bind(&user.balance)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .execute(&pool)
        .await?;
    println!("User added to database");
    
    // add to hashmap
    DAO_INFO.lock().unwrap().add_user(user);
    println!("User added to hashmap globals::users");
    Ok(())
}

/*pub async fn get_last_id() -> Result<i32, Box<dyn Error>> {

    let url = "postgres://postgres:password@localhost:5432/postgres";
    let pool = PgPoolOptions::new()
    .connect(url)
    .await?;

    // check if table is empty
    let q = "SELECT COUNT(*) FROM bank.users";
    let query_row_count: i32 = sqlx::query_scalar(q).fetch_one(&pool).await?;

    if query_row_count == 0 {
        return Ok(0)
    }

    //let q = "SELECT * ROW_NUMBER() OVER (ORDER BY ID) AS ID FROM bank.users";
    let q = "SELECT MAX(ID) FROM bank.users";
    let query = sqlx::query_scalar(q);

    let id: i32 = query.fetch_one(&pool).await?;

    Ok(id)
}*/