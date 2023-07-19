// api/src/models/project.rs
use diesel::Queryable;
use diesel::Insertable;
use crate::schema::projects;

#[derive(Queryable, Insertable)]
#[table_name="projects"]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Project {
    pub fn create(project: Project, connection: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(projects::table)
            .values(&project)
            .execute(connection)
    }

    pub fn read(connection: &MysqlConnection) -> QueryResult<Vec<Project>> {
        projects::table.load::<Project>(connection)
    }

    pub fn update(id: i32, project: Project, connection: &MysqlConnection) -> QueryResult<usize> {
        diesel::update(projects::table.find(id))
            .set(&project)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
        diesel::delete(projects::table.find(id))
            .execute(connection)
    }
}
