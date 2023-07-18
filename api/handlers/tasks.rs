use crate::models::Task;
use sqlx::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_tasks(db: web::Data<PoolType>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE created_by = $1")
        .bind(user_id)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_task(db: web::Data<PoolType>, task_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1 AND created_by = $2")
        .bind(task_id.into_inner())
        .bind(user_id)
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_task(db: web::Data<PoolType>, task: web::Json<Task>) -> impl Responder {
    let result = sqlx::query("INSERT INTO tasks (title, description, due_date, priority, status, project_id, list_id, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.due_date)
        .bind(&task.priority)
        .bind(&task.status)
        .bind(&task.project_id)
        .bind(&task.list_id)
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn update_task(db: web::Data<PoolType>, task_id: web::Path<i32>, task: web::Json<Task>) -> impl Responder {
    let result = sqlx::query("UPDATE tasks SET title = $1, description = $2, due_date = $3, priority = $4, status = $5, project_id = $6, list_id = $7 WHERE id = $8 AND created_by = $9")
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.due_date)
        .bind(&task.priority)
        .bind(&task.status)
        .bind(&task.project_id)
        .bind(&task.list_id)
        .bind(task_id.into_inner())
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete_task(db: web::Data<PoolType>, task_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND created_by = $2")
        .bind(task_id.into_inner())
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

// Implement create_task, get_task, update_task, delete_task similarly
    }

    // Implement create_task, get_task, update_task, delete_task similarly
    // Implement get_projects, get_project, create_project, update_project, delete_project similarly to tasks
