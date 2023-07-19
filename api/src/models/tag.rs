// api/src/models/tag.rs
#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}
