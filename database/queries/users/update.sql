UPDATE users
SET bio = $2
WHERE id = $1
RETURNING id, username, email, bio, created_at;

