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

    #[derive(Queryable)]
    pub struct List {
        pub id: i32,
        pub name: String,
        pub user_id: i32,
    }
    #[derive(Queryable)]
    pub struct Task {
        pub id: i32,
        pub title: String,
        pub description: String,
        pub due_date: DateTime<Utc>,
        pub priority: String,
        pub status: String,
        pub project_id: i32,
        pub list_id: i32,
        pub created_by: i32,
    }

    #[derive(Queryable)]
    pub struct Project {
        pub id: i32,
        pub name: String,
        pub description: String,
    }

    #[derive(Queryable)]
    pub struct List {
        pub id: i32,
        pub name: String,
        pub created_by: i32,
    }
