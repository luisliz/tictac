<<<<<<< HEAD
// api/src/models/tag.rs
use diesel::Queryable;
use crate::schema::tags;

#[derive(Queryable)]
=======
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::Queryable;
use diesel::Insertable;

#[derive(Queryable, Insertable)]
>>>>>>> 2034ec7 (aider: work in progress)
#[table_name="tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl Tag {
    pub fn create(tag: Tag, connection: &PgConnection) -> QueryResult<Tag> {
        diesel::insert_into(tags::table)
            .values(&tag)
            .get_result(connection)
    }

    pub fn read(connection: &PgConnection) -> QueryResult<Vec<Tag>> {
        tags::table.load::<Tag>(connection)
    }

    pub fn update(id: i32, tag: Tag, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(tags::table.find(id))
            .set(&tag)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(tags::table.find(id))
            .execute(connection)
    }
}
