-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE todos(
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    user_id integer NOT NULL REFERENCES users(id),
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
);
