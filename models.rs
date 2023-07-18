    use bcrypt::{hash, DEFAULT_COST};

    #[derive(Queryable)]
    pub struct Task {
        pub id: i32,
        pub title: String,
        pub completed: bool,
    }

    #[derive(Queryable)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub email: String,
        pub password_hash: String,
    }

    impl User {
        pub fn new(username: String, email: String, password: String) -> User {
            let password_hash = hash(password, DEFAULT_COST).unwrap();
            User { id: 0, username, email, password_hash }
        }
    }

    #[derive(Queryable)]
    pub struct List {
        pub id: i32,
        pub name: String,
        pub user_id: i32,
    }
