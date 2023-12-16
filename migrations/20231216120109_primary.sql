CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE items (
    id bigserial PRIMARY KEY,
    content TEXT NOT NULL,
    embedding vector(768)
);
