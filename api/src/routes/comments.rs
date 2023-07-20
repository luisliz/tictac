// api/src/routes/comments.rs
use crate::models::Comment;
use actix_web::{web, HttpResponse, Responder};

// Add routes for creating, getting, updating, and deleting comments
#[post("/comments")]
async fn create_comment(db: web::Data<Pool>, comment: web::Json<Comment>) -> impl Responder {
    // Code to create a new comment in the database
}

#[get("/comments/{id}")]
async fn get_comment(db: web::Data<Pool>, comment_id: web::Path<i32>) -> impl Responder {
    // Code to fetch a comment from the database
}

#[put("/comments/{id}")]
async fn update_comment(db: web::Data<Pool>, comment_id: web::Path<i32>, comment: web::Json<Comment>) -> impl Responder {
    // Code to update a comment in the database
}

#[delete("/comments/{id}")]
async fn delete_comment(db: web::Data<Pool>, comment_id: web::Path<i32>) -> impl Responder {
    // Code to delete a comment from the database
}
