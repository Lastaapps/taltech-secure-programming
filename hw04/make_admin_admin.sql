INSERT INTO roles_users (user_id, role_id) VALUES(
    (SELECT id FROM users WHERE username='admin'),
    (SELECT id FROM roles WHERE name='admin')
);
