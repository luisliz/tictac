use crate::models::List;
use sqlx::prelude::*;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_lists(db: web::Data<PoolType>) -> impl Responder {
    let result = sqlx::query_as::<_, List>("SELECT * FROM lists WHERE created_by = $1 AND archived = false")
        .bind(user_id)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        _ => HttpResponse::InternalServerError().into(),
    }
}

// Implement create_list, get_list, update_list, archive_list, unarchive_list, reorder_tasks, get_tasks_by_list similarly
    }

    // Implement get_list, create_list, update_list, delete_list similarly
