CREATE TABLE IF NOT EXISTS components
(   
    id INTEGER PRIMARY KEY,
    name    TEXT                    NOT NULL,
    size    TEXT,
    value   TEXT,
    info    TEXT,
    stock   INTEGER                 NOT NULL,
    origin  TEXT,
    url     TEXT,
    label   TEXT
);

