use bcrypt::{hash, DEFAULT_COST};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> User {
        let password_hash = hash(password, DEFAULT_COST).unwrap();
        User { id: 0, name, email, password_hash }
    }
}

