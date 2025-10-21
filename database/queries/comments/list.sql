SELECT id, author_id, post_id, content, created_at
FROM comments
WHERE post_id = $1
ORDER BY created_at ASC;

