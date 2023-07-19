    // Define the Comment model
    // This is a general outline, the actual implementation will depend on the specific requirements for the Comment model
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Comment {
        pub id: i32,
        pub task_id: i32,
        pub user_id: i32,
        pub content: String,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }
