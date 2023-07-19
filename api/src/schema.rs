// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        created_by -> Nullable<Integer>,
    }
}

diesel::table! {
    projects (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Nullable<Integer>,
        completed -> Nullable<Bool>,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        due_date -> Nullable<Timestamp>,
        priority -> Nullable<Text>,
        status -> Nullable<Text>,
        project_id -> Nullable<Integer>,
        list_id -> Nullable<Integer>,
        created_by -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Nullable<Text>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        password_hash -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    projects,
    tasks,
    users,
);
