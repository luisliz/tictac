use crate::models::Task;
use sqlx::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_tasks(db: web::Data<PoolType>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        _ => HttpResponse::InternalServerError().into(),
    }
}
    }

    // Implement create_task, get_task, update_task, delete_task similarly
    // Implement get_projects, get_project, create_project, update_project, delete_project similarly to tasks
