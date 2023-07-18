    // With sqlx, we don't need to define the schema in code. We can use SQL queries directly.

    table! {
        tasks (id) {
            id -> Int4,
            title -> Varchar,
            description -> Text,
            due_date -> Timestamp,
            priority -> Varchar,
            status -> Varchar,
            project_id -> Int4,
            list_id -> Int4,
            created_by -> Int4,
        }
    }

    table! {
        projects (id) {
            id -> Int4,
            name -> Varchar,
            description -> Text,
        }
    }

    table! {
        lists (id) {
            id -> Int4,
            name -> Varchar,
            created_by -> Int4,
        }
    }
