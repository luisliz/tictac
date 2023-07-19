// api/src/handlers/lists.rs
use crate::models::List;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use crate::db::Pool;

pub async fn get_lists(db: web::Data<Pool>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = List::read(&connection).await.expect("Failed to fetch lists");

    HttpResponse::Ok().json(result)
}

pub async fn create_list(db: web::Data<Pool>, list: web::Json<List>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = List::create(list.into_inner(), &connection).await.expect("Failed to create list");

    HttpResponse::Created().finish()
}

pub async fn update_list(db: web::Data<Pool>, list_id: web::Path<i32>, list: web::Json<List>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = List::update(list_id.into_inner(), list.into_inner(), &connection).await.expect("Failed to update list");

    HttpResponse::Ok().finish()
}

pub async fn delete_list(db: web::Data<Pool>, list_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = List::delete(list_id.into_inner(), &connection).await.expect("Failed to delete list");

    HttpResponse::NoContent().finish()
}

pub async fn reorder_tasks(db: web::Data<Pool>, list_id: web::Path<i32>, new_order: web::Json<Vec<i32>>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("UPDATE tasks SET order = new_order WHERE list_id = $1")
        .bind(list_id.into_inner())
        .execute(&connection)
        .expect("Failed to reorder tasks");

    HttpResponse::Ok().finish()
}

pub async fn get_tasks_by_list(db: web::Data<Pool>, list_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE list_id = $1 ORDER BY order")
        .bind(list_id.into_inner())
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}
