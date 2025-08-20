// this is like a class

use config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env::{self, VarError};

pub mod config;
pub mod schema;

// we can write from scratch how to connect to a db what bytes to send but we would be using a library
// sqlx or diesel

pub struct Store {
    pub conn: PgConnection,
}
// follow this up
// https://diesel.rs/guides/getting-started

impl Store {
    fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn: PgConnection = PgConnection::establish(&config.db_url)?;
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
