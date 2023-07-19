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

pub async fn create_project(db: Data<PoolType>, project: web::Json<Project>) -> impl Responder {
    let result = diesel::insert_into(projects::table)
        .values(&project.into_inner())
        .execute(&db.get().unwrap())
        .expect("Failed to create project");

    HttpResponse::Created().finish()
}

pub async fn get_project(db: Data<PoolType>, project_id: web::Path<i32>) -> impl Responder {
    let result = diesel::sql_query("SELECT * FROM projects WHERE id = $1")
        .bind(project_id.into_inner())
        .load::<Project>(&db.get().unwrap())
        .expect("Failed to fetch project");

    HttpResponse::Ok().json(result)
}

pub async fn update_project(db: Data<PoolType>, project_id: web::Path<i32>, project: web::Json<Project>) -> impl Responder {
    let result = diesel::sql_query("UPDATE projects SET name = $1, description = $2 WHERE id = $3")
        .bind(&project.name)
        .bind(&project.description)
        .bind(project_id.into_inner())
        .execute(&db.get().unwrap())
        .expect("Failed to update project");

    HttpResponse::Ok().finish()
}

pub async fn delete_project(db: Data<PoolType>, project_id: web::Path<i32>) -> impl Responder {
    let result = diesel::sql_query("DELETE FROM projects WHERE id = $1")
        .bind(project_id.into_inner())
        .execute(&db.get().unwrap())
        .expect("Failed to delete project");

    HttpResponse::NoContent().finish()
}
