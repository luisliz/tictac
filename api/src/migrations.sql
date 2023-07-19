CREATE TABLE tasks (
    id INT PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    due_date TIMESTAMP,
    priority VARCHAR(255),
    status VARCHAR(255),
    project_id INT,
    list_id INT,
    created_by INT
);

CREATE TABLE projects (
    id INT PRIMARY KEY,
    name VARCHAR(255),
    description TEXT
);

CREATE TABLE lists (
    id INT PRIMARY KEY,
    name VARCHAR(255),
    created_by INT
);

CREATE TABLE users (
    id INT PRIMARY KEY,
    username VARCHAR(255),
    name VARCHAR(255),
    email VARCHAR(255),
    password_hash VARCHAR(255)
);
