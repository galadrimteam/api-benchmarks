SELECT id, username, email, bio, created_at
FROM users
WHERE id = $1;

