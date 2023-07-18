use crate::models::Project;
use sqlx::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_projects(db: web::Data<PoolType>) -> impl Responder {
    let result = sqlx::query_as::<_, Project>("SELECT * FROM projects")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(projects) => HttpResponse::Ok().json(projects),
        _ => HttpResponse::InternalServerError().into(),
    }
}

// Implement create_project, get_project, update_project, delete_project similarly
    }

    // Implement get_project, create_project, update_project, delete_project similarly
