PRAGMA foreign_keys=ON;

CREATE TABLE IF NOT EXISTS component
(
    id INTEGER PRIMARY KEY,
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
    FOREIGN KEY(component_id)       REFERENCES component(id)
);

CREATE TABLE IF NOT EXISTS component_type
(

    component_id    INTEGER,
    type_id         INTEGER,

    PRIMARY KEY(component_id, type_id),
    
    FOREIGN KEY(component_id)   REFERENCES component(id),
    FOREIGN KEY(type_id)        REFERENCES types(id)
);


CREATE TABLE IF NOT EXISTS types
(
    id INTEGER PRIMARY KEY,
    name        TEXT            NOT NULL,
    inherits    INTEGER         NOT NULL
);

CREATE TABLE IF NOT EXISTS type_attribute
(
    -- id INTEGER PRIMARY KEY,
    
    type_id     INTEGER         NOT NULL,
    
    attributes  TEXT            NOT NULL,
    schema      TEXT            NOT NULL,
    prompts     TEXT            NOT NULL,

    PRIMARY KEY(type_id),
    FOREIGN KEY(type_id)        REFERENCES types(id)
);





-- Example

-- CREATE TABLE IF NOT EXISTS resistor
-- (
--     FOREIGN KEY(component_id)   REFERENCES component(id),
--     resistance  INTEGER,
--     accuracy    INTEGER
-- )