use crate::config::Config;
use diesel::prelude::*;
use std::env::{self, VarError};
pub struct Store {
    pub conn: PgConnection,
}
// follow this up
// https://diesel.rs/guides/getting-started

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn: PgConnection = PgConnection::establish(&config.db_url)?;
        Ok(Self { conn })
    }
}
