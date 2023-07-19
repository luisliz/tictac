use actix_web::{web, HttpResponse, Responder};
use crate::models::Tag;
use crate::PoolType;

pub async fn get_tags(db: web::Data<PoolType>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tags")
        .load::<Tag>(&connection)
        .expect("Failed to fetch tags");

    HttpResponse::Ok().json(result)
}

pub async fn create_tag(db: web::Data<PoolType>, tag: web::Json<Tag>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::insert_into(tags::table)
        .values(&tag.into_inner())
        .execute(&connection)
        .expect("Failed to create tag");

    HttpResponse::Created().finish()
}

pub async fn update_tag(db: web::Data<PoolType>, tag_id: web::Path<i32>, tag: web::Json<Tag>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("UPDATE tags SET name = $1 WHERE id = $2")
        .bind(&tag.name)
        .bind(tag_id.into_inner())
        .execute(&connection)
        .expect("Failed to update tag");

    HttpResponse::Ok().finish()
}

pub async fn delete_tag(db: web::Data<PoolType>, tag_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("DELETE FROM tags WHERE id = $1")
        .bind(tag_id.into_inner())
        .execute(&connection)
        .expect("Failed to delete tag");

    HttpResponse::NoContent().finish()
}

pub async fn get_tasks_by_tag(db: web::Data<PoolType>, tag_id: web::Path<i32>) -> impl Responder {
    let connection = db.get().unwrap();
    let result = diesel::sql_query("SELECT * FROM tasks WHERE id IN (SELECT task_id FROM task_tags WHERE tag_id = $1)")
        .bind(tag_id.into_inner())
        .load::<Task>(&connection)
        .expect("Failed to fetch tasks");

    HttpResponse::Ok().json(result)
}
