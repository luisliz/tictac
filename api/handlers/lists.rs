use crate::models::List;
use sqlx::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_lists(db: web::Data<PoolType>) -> impl Responder {
    let result = sqlx::query_as::<_, List>("SELECT * FROM lists")
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        _ => HttpResponse::InternalServerError().into(),
    }
}
    }

    // Implement get_list, create_list, update_list, delete_list similarly
