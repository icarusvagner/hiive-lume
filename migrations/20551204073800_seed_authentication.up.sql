
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
    '#01#fpqM8aqCiGUwQaF3SY-rluNw-sQH2GP7sIjL_XhjjMgL5aabpafrKuT-vlPvAhxSgt_Nl3B-MtmuEhi-apSoAg',
    gen_random_uuid(),                        -- pass_salt
    gen_random_uuid(),                        -- token_salt
    'active',                                 -- user_session_state
    0,
    NOW(),
    0,
    NOW()
)
ON CONFLICT DO NOTHING;
