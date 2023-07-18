    use crate::models::Project;
    use diesel::prelude::*;
    use actix_web::{web, HttpResponse, Responder};

    pub async fn get_projects(db: web::Data<PoolType>) -> impl Responder {
        use crate::schema::projects::dsl::*;

        let connection = db.get().unwrap();
        let result = projects.load::<Project>(&connection);

        match result {
            Ok(projects) => HttpResponse::Ok().json(projects),
            _ => HttpResponse::InternalServerError().into(),
        }
    }

    // Implement get_project, create_project, update_project, delete_project similarly
