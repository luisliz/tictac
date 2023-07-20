// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Nullable<Integer>,
        task_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        file_name -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    comments (id) {
        id -> Nullable<Integer>,
        task_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    lists (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        created_by -> Nullable<Integer>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Nullable<Integer>,
        task_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
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
    sessions (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        token -> Nullable<Text>,
        expires -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        color -> Nullable<Text>,
    }
}

diesel::table! {
    task_tags (task_id, tag_id) {
        task_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
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
    user_lists (user_id, list_id) {
        user_id -> Nullable<Integer>,
        list_id -> Nullable<Integer>,
    }
}

diesel::table! {
    user_projects (user_id, project_id) {
        user_id -> Nullable<Integer>,
        project_id -> Nullable<Integer>,
    }
}

diesel::table! {
    user_tags (user_id, tag_id) {
        user_id -> Nullable<Integer>,
        tag_id -> Nullable<Integer>,
    }
}

diesel::table! {
    user_tasks (user_id, task_id) {
        user_id -> Nullable<Integer>,
        task_id -> Nullable<Integer>,
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
    attachments,
    comments,
    lists,
    notifications,
    projects,
    sessions,
    tags,
    task_tags,
    tasks,
    user_lists,
    user_projects,
    user_tags,
    user_tasks,
    users,
);
