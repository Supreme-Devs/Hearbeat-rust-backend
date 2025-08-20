// this is like a class

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env::{self, VarError};

pub mod schema;
// pub mod config;

// we can write from scratch how to connect to a db what bytes to send but we would be using a library
// sqlx or diesel

pub struct Store {
    pub conn: PgConnection,
}
// follow this up
// https://diesel.rs/guides/getting-started

impl Store {
    fn default() -> Result<Self, ConnectionError> {
        let db_url: String = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("Please provide the DATABASE_URL environment variable"));

        let conn: PgConnection = PgConnection::establish(&db_url)?;
        Ok(Self { conn })
    }
}

impl Store {
    pub fn create_user(&self) {
        println!("create user");
    }

    pub fn create_website(&self) -> String {
        String::from("1")
    }
}
