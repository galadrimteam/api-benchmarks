SELECT id, username, email, bio, created_at
FROM users
ORDER BY created_at DESC
LIMIT $1 OFFSET $2;

