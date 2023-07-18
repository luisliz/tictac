use actix_web::{web, HttpResponse, Responder};
use crate::PoolType;

pub async fn search(db: web::Data<PoolType>, query: web::Query<String>) -> impl Responder {
    // Perform a full text search on task titles, descriptions, notes, etc.
}
