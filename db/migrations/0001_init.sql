PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS component
(
    component_id INTEGER PRIMARY KEY,
    name            TEXT            NOT NULL,
    stock           INTEGER         NOT NULL,
    price           FLOAT,
    manufacturer    TEXT,
    label           TEXT,
    image           BOOLEAN         NOT NULL,
    datasheet       BOOLEAN         NOT NULL
);


CREATE TABLE IF NOT EXISTS origin
(
    id INTEGER PRIMARY KEY,
    origin          TEXT            NOT NULL,
    part_number     TEXT,
    component_id    INTEGER         NOT NULL,
    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_type
(

    component_id    INTEGER,
    type_id         INTEGER,

    attributes      TEXT        NOT NULL,

    PRIMARY KEY(component_id, type_id),
    
    FOREIGN KEY(component_id)   REFERENCES component(component_id)  ON DELETE CASCADE,
    FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS type
(
    type_id INTEGER PRIMARY KEY,
    name        TEXT            NOT NULL,
    inherits    INTEGER         NOT NULL
);

CREATE TABLE IF NOT EXISTS type_attribute
(
    
    type_id     INTEGER         NOT NULL,
    
    fields      TEXT            NOT NULL,
    schema      TEXT            NOT NULL,
    prompts     TEXT            NOT NULL,

    PRIMARY KEY(type_id),
    FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
);