use diesel::prelude::*;
use diesel::Connection;
use diesel::Queryable;
use diesel::Insertable;
use crate::schema::projects;

#[derive(Queryable, Insertable)]
#[diesel(table_name=projects)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn create<C: Connection>(project: Project, connection: &C) -> QueryResult<Project> {
        diesel::insert_into(projects::table)
            .values(&project)
            .get_result(connection)
    }

    pub fn read<C: Connection>(connection: &C) -> QueryResult<Vec<Project>> {
        projects::table.load::<Project>(connection)
    }

    pub fn update<C: Connection>(id: i32, project: Project, connection: &C) -> QueryResult<usize> {
        diesel::update(projects::table.find(id))
            .set(&project)
            .execute(connection)
    }

    pub fn delete<C: Connection>(id: i32, connection: &C) -> QueryResult<usize> {
        diesel::delete(projects::table.find(id))
            .execute(connection)
    }
}
