use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod models;
mod schema;
mod auth;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::PgConnection;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(|| {
        App::new()
            .data(database_url.clone())
            .route("/users", web::post().to(auth::signup))
            .route("/login", web::post().to(auth::login))
            .route("/tasks", web::get().to(handlers::get_tasks))
            .route("/tasks/{id}", web::get().to(handlers::get_task))
            .route("/tasks", web::post().to(handlers::create_task))
            .route("/tasks/{id}", web::put().to(handlers::update_task))
            .route("/tasks/{id}", web::delete().to(handlers::delete_task))
            .route("/projects", web::get().to(handlers::get_projects))
            .route("/projects/{id}", web::get().to(handlers::get_project))
            .route("/projects", web::post().to(handlers::create_project))
            .route("/projects/{id}", web::put().to(handlers::update_project))
            .route("/projects/{id}", web::delete().to(handlers::delete_project))
            .route("/lists", web::get().to(handlers::get_lists))
            .route("/lists/{id}", web::get().to(handlers::get_list))
            .route("/lists", web::post().to(handlers::create_list))
            .route("/lists/{id}", web::put().to(handlers::update_list))
            .route("/lists/{id}", web::delete().to(handlers::delete_list))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
