#[derive(Queryable)]
pub struct List {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}
