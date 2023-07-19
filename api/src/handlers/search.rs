// api/src/handlers/search.rs
use actix_web::{web, HttpResponse, Responder};
use crate::PoolType;
use crate::models::Task;
use diesel::prelude::*;

pub async fn search(db: web::Data<PoolType>, query: web::Query<String>) -> impl Responder {
    use crate::schema::tasks::dsl::*;

    let connection = db.get_ref().get().unwrap();
    let results = tasks.filter(title.like(format!("%{}%", query.into_inner())))
        .load::<Task>(&connection)
        .expect("Error loading tasks");

    HttpResponse::Ok().json(results)
}
