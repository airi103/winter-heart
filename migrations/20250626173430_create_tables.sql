-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL
);

CREATE TABLE IF NOT EXISTS items (
    user_id INTEGER NOT NULL,
    item TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE (user_id, item)  -- required for ON CONFLICT clause
);
