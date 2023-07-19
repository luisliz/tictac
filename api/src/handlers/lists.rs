// api/src/handlers/lists.rs
use crate::models::list;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use meilisearch_sdk::tasks::Task;

pub async fn get_lists(db: web::Data<PoolType>) -> impl Responder {
    let result = List::read(&db.get().unwrap()).await.expect("Failed to fetch lists");

    HttpResponse::Ok().json(result)
}

pub async fn create_list(db: web::Data<PoolType>, list: web::Json<List>) -> impl Responder {
    let result = List::create(list.into_inner(), &db.get().unwrap()).await.expect("Failed to create list");

    HttpResponse::Created().finish()
}

pub async fn update_list(db: web::Data<PoolType>, list_id: web::Path<i32>, list: web::Json<List>) -> impl Responder {
    let result = List::update(list_id.into_inner(), list.into_inner(), &db.get().unwrap()).await.expect("Failed to update list");

    HttpResponse::Ok().finish()
}

pub async fn delete_list(db: web::Data<PoolType>, list_id: web::Path<i32>) -> impl Responder {
    let result = List::delete(list_id.into_inner(), &db.get().unwrap()).await.expect("Failed to delete list");

    HttpResponse::NoContent().finish()
}

pub async fn reorder_tasks(db: web::Data<PoolType>, list_id: web::Path<i32>, new_order: web::Json<Vec<i32>>) -> impl Responder {
    let result = diesel::sql_query("UPDATE tasks SET order = new_order WHERE list_id = $1")
        .bind(list_id.into_inner())
        .execute(&db.get().unwrap())
        .expect("Failed to reorder tasks");

    HttpResponse::Ok().finish()
}

pub async fn get_tasks_by_list(db: web::Data<PoolType>, list_id: web::Path<i32>) -> impl Responder {
    let result = diesel::sql_query("SELECT * FROM tasks WHERE list_id = $1 ORDER BY order")
        .bind(list_id.into_inner())
        .load::<Task>(&db.get().unwrap())
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}

// Implement get_list, create_list, update_list, delete_list similarly
