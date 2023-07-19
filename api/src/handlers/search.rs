use actix_web::{web, HttpResponse, Responder};
use crate::PoolType;

use meilisearch_sdk::search::Query;
use meilisearch_sdk::client::*;

pub async fn search(db: web::Data<PoolType>, query: web::Query<String>) -> impl Responder {
    let client = Client::new("http://localhost:7700", "masterKey");
    let tasks = client.get_or_create("tasks").await.unwrap();

    let results = tasks.search()
        .with_query(&query)
        .execute::<Task>()
        .await
        .unwrap();

    HttpResponse::Ok().json(results.hits)
}
