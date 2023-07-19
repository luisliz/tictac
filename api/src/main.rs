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

fn initialize_db_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL should be set");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(conn_spec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = initialize_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::post().to(auth::signup))
            .route("/login", web::post().to(auth::login))
            .route("/tasks", web::get().to(handlers::tasks::get_tasks))
            .route("/tasks/{id}", web::get().to(handlers::tasks::get_task))
            .route("/tasks", web::post().to(handlers::tasks::create_task))
            .route("/tasks/{id}", web::put().to(handlers::tasks::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::tasks::delete_task))
            .route("/projects", web::get().to(handlers::projects::get_projects))
            // .route("/projects/{id}", web::get().to(handlers::projects::get_project))
            // .route("/projects", web::post().to(handlers::projects::create_project))
            // .route("/projects/{id}", web::put().to(handlers::projects::update_project))
            // .route("/projects/{id}", web::delete().to(handlers::projects::delete_project))
            .route("/lists", web::get().to(handlers::lists::get_lists))
            // .route("/lists/{id}", web::get().to(handlers::lists::get_list))
            // .route("/lists", web::post().to(handlers::lists::create_list))
            // .route("/lists/{id}", web::put().to(handlers::lists::update_list))
            // .route("/lists/{id}", web::delete().to(handlers::lists::delete_list))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    .expect("Http server failed");

    Ok(())
}
