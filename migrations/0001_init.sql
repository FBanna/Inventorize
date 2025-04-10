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

CREATE TABLE IF NOT EXISTS name_prompt(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS size_prompt(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS value_prompt(    entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS info_prompt(     entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS origin_prompt(   entry   TEXT    NOT NULL);
CREATE TABLE IF NOT EXISTS label_prompt(    entry   TEXT    NOT NULL);