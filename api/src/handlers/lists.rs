use crate::models::list;
use sqlx::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_lists(db: web::Data<PoolType>) -> impl Responder {
    let result = List::read(db.get_ref()).await;

    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_list(db: web::Data<PoolType>, list: web::Json<List>) -> impl Responder {
    let result = List::create(list.into_inner(), db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn update_list(db: web::Data<PoolType>, list_id: web::Path<i32>, list: web::Json<List>) -> impl Responder {
    let result = List::update(list_id.into_inner(), list.into_inner(), db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete_list(db: web::Data<PoolType>, list_id: web::Path<i32>) -> impl Responder {
    let result = List::delete(list_id.into_inner(), db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn reorder_tasks(db: web::Data<PoolType>, list_id: web::Path<i32>, new_order: web::Json<Vec<i32>>) -> impl Responder {
    let result = sqlx::query("UPDATE tasks SET order = new_order WHERE list_id = $1")
        .bind(list_id.into_inner())
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().into(),
        _ => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_tasks_by_list(db: web::Data<PoolType>, list_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE list_id = $1 ORDER BY order")
        .bind(list_id.into_inner())
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        _ => HttpResponse::InternalServerError().into(),
    }
}
    }

    // Implement get_list, create_list, update_list, delete_list similarly
