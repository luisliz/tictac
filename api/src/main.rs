use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::db::Pool;

mod auth;
mod handlers;
mod models;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
    .expect("Http server failed");

    Ok(())
}
