// api/src/handlers/projects.rs
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use web::Data;

pub async fn get_projects(db: Data<PoolType>) -> impl Responder {
    let result = diesel::sql_query("SELECT * FROM projects")
        .load::<Project>(&db.get().unwrap())
        .expect("Failed to fetch projects");

    HttpResponse::Ok().json(result)
}

// Implement create_project, get_project, update_project, delete_project similarly
