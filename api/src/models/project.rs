use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Queryable;
use diesel::Insertable;

#[derive(Queryable, Insertable)]
#[table_name="projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn create(project: Project, connection: &PgConnection) -> QueryResult<Project> {
        diesel::insert_into(projects::table)
            .values(&project)
            .get_result(connection)
    }

    pub fn read(connection: &PgConnection) -> QueryResult<Vec<Project>> {
        projects::table.load::<Project>(connection)
    }

    pub fn update(id: i32, project: Project, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(projects::table.find(id))
            .set(&project)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(projects::table.find(id))
            .execute(connection)
    }
}
