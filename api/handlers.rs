    use crate::models::Task;
    use diesel::prelude::*;
    use actix_web::{web, HttpResponse, Responder};

    pub async fn get_tasks(db: web::Data<PoolType>) -> impl Responder {
        use crate::schema::tasks::dsl::*;

        let connection = db.get().unwrap();
        let result = tasks.load::<Task>(&connection);

        match result {
            Ok(tasks) => HttpResponse::Ok().json(tasks),
            _ => HttpResponse::InternalServerError().into(),
        }
    }

    // Implement create_task, get_task, update_task, delete_task similarly
