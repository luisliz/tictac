// api/src/models/list.rs
use diesel::Queryable;
use diesel::Insertable;
use diesel::prelude::*;
use diesel::Connection;
use crate::schema::lists;

#[derive(Queryable, Insertable)]
#[table_name="lists"]
pub struct List {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

impl List {
    pub fn create<C: Connection>(list: List, connection: &C) -> QueryResult<List> {
        diesel::insert_into(lists::table)
            .values(&list)
            .get_result(connection)
    }

    pub fn read<C: Connection>(connection: &C) -> QueryResult<Vec<List>> {
        lists::table.load::<List>(connection)
    }

    pub fn update<C: Connection>(id: i32, list: List, connection: &C) -> QueryResult<usize> {
        diesel::update(lists::table.find(id))
            .set(&list)
            .execute(connection)
    }

    pub fn delete<C: Connection>(id: i32, connection: &C) -> QueryResult<usize> {
        diesel::delete(lists::table.find(id))
            .execute(connection)
    }
}
