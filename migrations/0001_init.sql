CREATE TABLE IF NOT EXISTS components
(   
    id INTEGER PRIMARY KEY,
    name    TEXT                    NOT NULL,
    size    TEXT,
    value   TEXT,
    info    TEXT,
    stock   INTEGER                 NOT NULL,
    origin  TEXT,
    label   TEXT
);

CREATE TABLE IF NOT EXISTS namep(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS sizep(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS valuep(    entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS infop(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS originp(   entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS labelp(    entry   TEXT    NOT NULL);