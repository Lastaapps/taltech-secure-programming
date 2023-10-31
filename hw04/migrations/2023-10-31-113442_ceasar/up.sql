CREATE TABLE ceasar(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    shift INTEGER NOT NULL,
    data TEXT NOT NULL, -- base64 encoded
    deleted INTEGER DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)