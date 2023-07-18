#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub completed: bool,
    pub title: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub priority: String,
    pub status: String,
    pub project_id: i32,
    pub list_id: i32,
    pub created_by: i32,
}
