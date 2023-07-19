// api/src/handlers/projects.rs
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use web::Data;
use crate::db::Pool;
use crate::models::project::Project;

pub async fn get_projects(db: Data<Pool>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM projects")
        .load::<Project>(&connection)
        .expect("Failed to fetch projects");

    HttpResponse::Ok().json(result)
}

pub async fn create_project(db: Data<Pool>, project: web::Json<Project>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::insert_into(projects::table)
        .values(&project.into_inner())
        .execute(&connection)
        .expect("Failed to create project");

    HttpResponse::Created().finish()
}

pub async fn get_project(db: Data<Pool>, project_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM projects WHERE id = $1")
        .bind(project_id.into_inner())
        .load::<Project>(&connection)
        .expect("Failed to fetch project");

    HttpResponse::Ok().json(result)
}

pub async fn update_project(db: Data<Pool>, project_id: web::Path<i32>, project: web::Json<Project>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("UPDATE projects SET name = $1, description = $2 WHERE id = $3")
        .bind(&project.name)
        .bind(&project.description)
        .bind(project_id.into_inner())
        .execute(&connection)
        .expect("Failed to update project");

    HttpResponse::Ok().finish()
}

pub async fn delete_project(db: Data<Pool>, project_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("DELETE FROM projects WHERE id = $1")
        .bind(project_id.into_inner())
        .execute(&connection)
        .expect("Failed to delete project");

    HttpResponse::NoContent().finish()
}
