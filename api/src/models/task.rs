use diesel::Queryable;
use diesel::Insertable;
use chrono::{DateTime, Utc};
use crate::schema::tasks;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Queryable, Insertable)]
#[diesel(table_name=tasks)]
pub struct Task {
    pub id: i32,
    pub completed: bool,
    pub title: String,
    pub description: String,
    // pub due_date: Option<DateTime<Utc>>,
    pub priority: String,
    pub status: String,
    pub project_id: i32,
    pub list_id: i32,
    pub created_by: i32,
}

impl Task {
    pub fn create(task: Task, connection: &PgConnection) -> QueryResult<Task> {
        diesel::insert_into(tasks::table)
            .values(&task)
            .get_result(connection)
    }

    pub fn read(connection: &PgConnection) -> QueryResult<Vec<Task>> {
        tasks::table.load::<Task>(connection)
    }

    pub fn update(id: i32, task: Task, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(tasks::table.find(id))
            .set(&task)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(tasks::table.find(id))
            .execute(connection)
    }
}
