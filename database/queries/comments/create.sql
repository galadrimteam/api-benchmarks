INSERT INTO comments (author_id, post_id, content)
VALUES ($1, $2, $3)
RETURNING id, author_id, post_id, content, created_at;

