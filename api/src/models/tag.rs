// api/src/models/tag.rs
use diesel::Queryable;
use crate::schema::tags;

#[derive(Queryable)]
#[table_name="tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}
