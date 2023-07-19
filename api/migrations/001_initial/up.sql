CREATE TABLE tasks
(
    id          INT PRIMARY KEY,
    completed   BOOLEAN,
    title       VARCHAR(255),
    description TEXT,
    due_date    TIMESTAMP,
    priority    VARCHAR(255),
    status      VARCHAR(255),
    project_id  INT,
    list_id     INT,
    created_by  INT
);

CREATE TABLE projects
(
    id          INT PRIMARY KEY,
    name        VARCHAR(255),
    description TEXT
);

CREATE TABLE lists
(
    id         INT PRIMARY KEY,
    name       VARCHAR(255),
    created_by INT
);

CREATE TABLE users
(
    id            INT PRIMARY KEY,
    username      VARCHAR(255),
    name          VARCHAR(255),
    email         VARCHAR(255),
    password_hash VARCHAR(255)
);

CREATE TABLE user_projects
(
    user_id    INT,
    project_id INT,
    PRIMARY KEY (user_id, project_id)
);

CREATE TABLE user_lists
(
    user_id INT,
    list_id INT,
    PRIMARY KEY (user_id, list_id)
);

CREATE TABLE user_tasks
(
    user_id INT,
    task_id INT,
    PRIMARY KEY (user_id, task_id)
);

CREATE TABLE tags
(
    id   INT PRIMARY KEY,
    name VARCHAR(255)
);

CREATE TABLE task_tags
(
    task_id INT,
    tag_id  INT,
    PRIMARY KEY (task_id, tag_id)
);

CREATE TABLE comments
(
    id         INT PRIMARY KEY,
    task_id    INT,
    user_id    INT,
    content    TEXT,
    created_at TIMESTAMP
);

CREATE TABLE attachments
(
    id         INT PRIMARY KEY,
    task_id    INT,
    user_id    INT,
    file_name  VARCHAR(255),
    created_at TIMESTAMP
);

CREATE TABLE notifications
(
    id         INT PRIMARY KEY,
    task_id    INT,
    user_id    INT,
    content    TEXT,
    created_at TIMESTAMP
);

CREATE TABLE sessions
(
    id        INT PRIMARY KEY,
    user_id   INT,
    token     VARCHAR(255),
    expires   TIMESTAMP,
    created_at TIMESTAMP
);