CREATE TABLE roles_users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULl,
    role_id INTEGER NOT NULl,
    FOREIGN KEY(user_id) REFERENCES users(id),
    FOREIGN KEY(role_id) REFERENCES roles(id)
);
