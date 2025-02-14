CREATE TABLE IF NOT EXISTS dialog (
    id VARCHAR(25) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(255) NOT NULL,
    description TEXT NULL,
    image VARCHAR(255) NULL,
    date_create TIMESTAMPTZ,
    date_update TIMESTAMPTZ,
    user_id VARCHAR(25) REFERENCES users(id)
);
