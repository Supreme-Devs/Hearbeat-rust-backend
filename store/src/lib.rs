// this is like a class
pub struct Store {
    conn: Connection,
}

// we can write from scratch how to connect to a db what bytes to send but we would be using a library
// sqlx or diesel

// follow this up
// https://diesel.rs/guides/getting-started

impl Default for Store {
    // this is like the constructor function which is called when the object is created
    fn default() -> Self {
        Self {
            conn: Connection::open("sqlite:///db.sqlite").unwrap(),
        }
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
