CREATE TABLE IF NOT EXISTS components
(   
    id INTEGER PRIMARY KEY,
    name    TEXT                    NOT NULL,
    size    TEXT,
    value   TEXT,
    info    TEXT,
    stock   INTEGER                 NOT NULL,
    origin  TEXT,
    label   TEXT,
    image   BOOLEAN                 NOT NULL,
    datasheet BOOLEAN               NOT NULL

);

CREATE TABLE IF NOT EXISTS name(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS size(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS value(    entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS info(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS origin(   entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS label(    entry   TEXT    NOT NULL);