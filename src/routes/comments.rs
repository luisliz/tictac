    // Add routes for creating, getting, updating, and deleting comments
    // This is a general outline, the actual implementation will depend on the specific requirements for each operation
    #[post("/comments")]
    async fn create_comment() {
        // Implementation goes here
    }

    #[get("/comments/<id>")]
    async fn get_comment(id: i32) {
        // Implementation goes here
    }

    #[put("/comments/<id>")]
    async fn update_comment(id: i32) {
        // Implementation goes here
    }

    #[delete("/comments/<id>")]
    async fn delete_comment(id: i32) {
        // Implementation goes here
    }
