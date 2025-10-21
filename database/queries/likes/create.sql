-- INSERT INTO post_likes (user_id, post_id) VALUES ($1, $2);

INSERT INTO post_likes (user_id, post_id)
VALUES ($1, $2)
ON CONFLICT DO NOTHING;