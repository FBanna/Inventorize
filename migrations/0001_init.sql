CREATE TABLE IF NOT EXISTS components
(
    ID      INTEGER PRIMARY KEY     NOT NULL,
    NAME    TEXT                    NOT NULL,
    SIZE    TEXT,
    INFO    TEXT,
    STOCK   INTEGER                 NOT NULL,
    ORIGIN  TEXT,
    URL     TEXT
);

