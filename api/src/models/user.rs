use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use diesel::Connection;
use diesel::Queryable;
use diesel::Insertable;
use crate::schema::users;

#[derive(Queryable, Insertable)]
#[diesel(table_name=users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, name: String, email: String, password: String) -> User {
        let password_hash = hash(password, DEFAULT_COST).unwrap();
        User { id: 0, username, name, email, password_hash }
    }
    pub fn create<C: Connection>(user: User, connection: &C) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(&user)
            .get_result(connection)
    }

    pub fn read<C: Connection>(connection: &C) -> QueryResult<Vec<User>> {
        users::table.load::<User>(connection)
    }

    pub fn update<C: Connection>(id: i32, user: User, connection: &C) -> QueryResult<usize> {
        diesel::update(users::table.find(id))
            .set(&user)
            .execute(connection)
    }

    pub fn delete<C: Connection>(id: i32, connection: &mut C) -> QueryResult<usize> {
        diesel::delete(users::table.find(id))
            .execute(connection)
    }
}
