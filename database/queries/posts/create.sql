INSERT INTO posts (author_id, content)
VALUES ($1, $2)
RETURNING id, author_id, content, created_at;

