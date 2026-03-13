

CREATE TABLE IF NOT EXISTS component
(
    id INTEGER PRIMARY KEY,
    name            TEXT            NOT NULL,
    stock           INTEGER         NOT NULL,
    price           FLOAT,
    manufacturer    TEXT,
    label           TEXT,
    image           BOOLEAN         NOT NULL,
    datasheet       BOOLEAN         NOT NULL,
);


CREATE TABLE IF NOT EXISTS origin
(
    FOREIGN KEY(component_id)   REFERENCES component(id),
    origin      TEXT            NOT NULL,
    part_number TEXT
);

CREATE TABLE IF NOT EXISTS component_type
(
    FOREIGN KEY(component_id)   REFERENCES component(id),
    FOREIGN KEY(type_id)        REFERENCES types(id)
)





CREATE TABLE IF NOT EXISTS types
(
    id INTEGER PRIMARY KEY,
    name        TEXT            NOT NULL,
    inherits    INTEGER         NOT NULL
);

CREATE TABLE IF NOT EXISTS type_attribute
(
    -- id INTEGER PRIMARY KEY,
    FOREIGN KEY(type_id)        REFERENCES types(id),
    attributes  TEXT            NOT NULL,
    schema      TEXT            NOT NULL,
    prompts     TEXT            NOT NULL
);





-- Example

-- CREATE TABLE IF NOT EXISTS resistor
-- (
--     FOREIGN KEY(component_id)   REFERENCES component(id),
--     resistance  INTEGER,
--     accuracy    INTEGER
-- )