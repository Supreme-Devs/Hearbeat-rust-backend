use crate::store::Store;

impl Store {
    pub fn create_user(&self) {
        println!("create user");
    }

    pub fn get_user(&self) -> String {
        String::from("1")
    }
}
