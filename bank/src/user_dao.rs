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
    let url = "postgres://postgres:password@localhost:5432/postgres";
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