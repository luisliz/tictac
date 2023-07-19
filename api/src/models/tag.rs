use diesel::Queryable;

#[derive(Queryable)]
#[table_name="tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
}
