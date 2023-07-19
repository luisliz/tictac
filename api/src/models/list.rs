// api/src/models/list.rs
use diesel::Queryable;
use diesel::Insertable;
use diesel::prelude::*;
use diesel::pg::PgConnection;

#[derive(Queryable, Insertable)]
#[table_name="lists"]
pub struct List {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

impl List {
    pub fn create(list: List, connection: &PgConnection) -> QueryResult<List> {
        diesel::insert_into(lists::table)
            .values(&list)
            .get_result(connection)
    }

    pub fn read(connection: &PgConnection) -> QueryResult<Vec<List>> {
        lists::table.load::<List>(connection)
    }

    pub fn update(id: i32, list: List, connection: &PgConnection) -> QueryResult<usize> {
        diesel::update(lists::table.find(id))
            .set(&list)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
        diesel::delete(lists::table.find(id))
            .execute(connection)
    }
}
