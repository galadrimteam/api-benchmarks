SELECT p.id,
       p.author_id,
       p.content,
       p.created_at,
       COALESCE(l.cnt, 0) AS like_count
FROM posts p
LEFT JOIN (
    SELECT post_id, COUNT(*) AS cnt
    FROM post_likes
    GROUP BY post_id
) l ON l.post_id = p.id
ORDER BY p.created_at DESC
LIMIT $1 OFFSET $2;

