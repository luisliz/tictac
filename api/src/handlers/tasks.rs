// api/src/handlers/tasks.rs
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use crate::db::Pool;
use crate::models::task::Task;

pub async fn get_tasks(db: web::Data<Pool>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE created_by = $1")
        .bind(user_id)
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}

pub async fn get_tasks_by_user(db: web::Data<Pool>, user_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE created_by = $1")
        .bind(user_id.into_inner())
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}

pub async fn get_tasks_by_list(db: web::Data<Pool>, list_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE list_id = $1")
        .bind(list_id.into_inner())
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}

pub async fn get_tasks_by_project(db: web::Data<Pool>, project_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE project_id = $1")
        .bind(project_id.into_inner())
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}

pub async fn get_tasks_by_status(db: web::Data<Pool>, status: web::Path<String>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE status = $1")
        .bind(status.into_inner())
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}

pub async fn get_task(db: web::Data<Pool>, task_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE id = $1 AND created_by = $2")
        .bind(task_id.into_inner())
        .bind(user_id)
        .load::<Task>(&connection)
        .expect("Failed to fetch task");

    HttpResponse::Ok().json(result)
}

pub async fn create_task(db: web::Data<Pool>, task: web::Json<Task>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("INSERT INTO tasks (title, description, due_date, priority, status, project_id, list_id, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.due_date)
        .bind(&task.priority)
        .bind(&task.status)
        .bind(&task.project_id)
        .bind(&task.list_id)
        .bind(user_id)
        .execute(&connection)
        .expect("Failed to create task");

    HttpResponse::Created().finish()
}

pub async fn update_task(db: web::Data<Pool>, task_id: web::Path<i32>, task: web::Json<Task>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("UPDATE tasks SET title = $1, description = $2, due_date = $3, priority = $4, status = $5, project_id = $6, list_id = $7 WHERE id = $8 AND created_by = $9")
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.due_date)
        .bind(&task.priority)
        .bind(&task.status)
        .bind(&task.project_id)
        .bind(&task.list_id)
        .bind(task_id.into_inner())
        .bind(user_id)
        .execute(&connection)
        .expect("Failed to update task");

    HttpResponse::Ok().finish()
}

pub async fn delete_task(db: web::Data<Pool>, task_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("DELETE FROM tasks WHERE id = $1 AND created_by = $2")
        .bind(task_id.into_inner())
        .bind(user_id)
        .execute(&connection)
        .expect("Failed to delete task");

    HttpResponse::NoContent().finish()
}
