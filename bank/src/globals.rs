use std::sync::Mutex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use crate::user::User;
//use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::error::Error;
use std::sync::atomic::AtomicUsize;


//static MYSQL_DB: OnceCell<Pool<Postgres>> = OnceCell::new();
pub struct DaoInfo {
    //pool: sqlx::Pool<Postgres>,
    pub users: HashMap<i32, User>,
}

impl DaoInfo {
    /*pub async fn db_connection(&self) -> Result<(), Box<dyn Error>> {
        let url = "postgres://postgres:password@localhost:5432/postgres";
        self.pool = PgPoolOptions::new()
        .connect(url)
        .await?;
        
        Ok(());
    }*/
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
    pub fn get_len(self) -> usize {
        self.users.keys().len()
    }
}

lazy_static! {
    pub static ref DAO_INFO: Mutex<DaoInfo> =
        Mutex::new(DaoInfo {
            //pool: PgPoolOptions::new().connect("postgres://postgres:password@localhost:5432/postgres").await?,
            users: HashMap::new()
        });
}