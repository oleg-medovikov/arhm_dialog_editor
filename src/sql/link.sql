CREATE TABLE IF NOT EXISTS link (
    "from" VARCHAR(25) REFERENCES node(id),
    "to" VARCHAR(25) REFERENCES node(id),
    class VARCHAR(20) NULL,
    text VARCHAR(255) NOT NULL,
    points REAL[],
    dialog_id VARCHAR(25) REFERENCES dialog(id)
);
