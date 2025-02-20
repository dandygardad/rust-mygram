CREATE TABLE IF NOT EXISTS photos(
    id SERIAL PRIMARY KEY,
    title VARCHAR(255),
    caption TEXT,
    photo_url VARCHAR(255),
    user_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);