
-- Ensure pgcrypto is installed for password hashing
CREATE EXTENSION IF NOT EXISTS pgcrypto;

INSERT INTO tbl_user_account (
    user_id,
    username,
    password_hash,
    pass_salt,
    token_salt,
    status,
    cid,
    ctime,
    mid,
    mtime
)
VALUES (
    'admin-001',                              -- user_id
    'admin',                                  -- username
    '#01#1ZUacqPZw7NVYvep6IDn2HJ00O8WBo1e0zvjNxWddEb4OGmkwUW2VssKC-IMjWc_himzKr6SaoprB7ufh204KQ',
    gen_random_uuid(),                        -- pass_salt
    gen_random_uuid(),                        -- token_salt
    'active',                                 -- user_session_state
    0,
    NOW(),
    0,
    NOW()
)
ON CONFLICT DO NOTHING;
