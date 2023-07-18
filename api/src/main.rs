use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod handlers;
mod schema;
mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
use sqlx::postgres::PgPoolOptions;

dotenv().ok();

let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool.");

//let meilisearch_url = env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set");
//let meilisearch_client = Client::new(&meilisearch_url, "");

    HttpServer::new(|| {
        App::new()
            .data(database_url.clone())
            .route("/users", web::post().to(auth::signup))
            .route("/login", web::post().to(auth::login))
            .route("/tasks", web::get().to(handlers::tasks::get_tasks))
            .route("/tasks/{id}", web::get().to(handlers::tasks::get_task))
            .route("/tasks", web::post().to(handlers::tasks::create_task))
            .route("/tasks/{id}", web::put().to(handlers::tasks::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::tasks::delete_task))
            .route("/projects", web::get().to(handlers::projects::get_projects))
            .route("/projects/{id}", web::get().to(handlers::projects::get_project))
            .route("/projects", web::post().to(handlers::projects::create_project))
            .route("/projects/{id}", web::put().to(handlers::projects::update_project))
            .route("/projects/{id}", web::delete().to(handlers::projects::delete_project))
            .route("/lists", web::get().to(handlers::lists::get_lists))
            .route("/lists/{id}", web::get().to(handlers::lists::get_list))
            .route("/lists", web::post().to(handlers::lists::create_list))
            .route("/lists/{id}", web::put().to(handlers::lists::update_list))
            .route("/lists/{id}", web::delete().to(handlers::lists::delete_list))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
use crate::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = db::connect().await?;
    db::migrate(&pool).await?;
    // rest of your application startup code
    Ok(())
}
