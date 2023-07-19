#[derive(Queryable, Insertable)]
#[table_name="lists"]
pub struct List {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

impl List {
    pub fn create(list: List, connection: &MysqlConnection) -> QueryResult<usize> {
        diesel::insert_into(lists::table)
            .values(&list)
            .execute(connection)
    }

    pub fn read(connection: &MysqlConnection) -> QueryResult<Vec<List>> {
        lists::table.load::<List>(connection)
    }

    pub fn update(id: i32, list: List, connection: &MysqlConnection) -> QueryResult<usize> {
        diesel::update(lists::table.find(id))
            .set(&list)
            .execute(connection)
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
        diesel::delete(lists::table.find(id))
            .execute(connection)
    }
}
