CREATE TABLE IF NOT EXISTS node (
    id VARCHAR(25) PRIMARY KEY,
    class VARCHAR(20) NULL,
    text TEXT NOT NULL,
    loc VARCHAR(50),
    date_create TIMESTAMPTZ,
    date_update TIMESTAMPTZ,
    active BOOLEAN,
    dialog_id VARCHAR(25) REFERENCES dialog(id),
    user_id VARCHAR(25) REFERENCES users(id)
);
