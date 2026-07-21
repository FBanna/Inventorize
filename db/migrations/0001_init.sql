CREATE EXTENSION IF NOT EXISTS ltree;


-- Components
CREATE TABLE IF NOT EXISTS component
(
    component_id    uuid DEFAULT uuidv7()   PRIMARY KEY,

    name            TEXT            NOT NULL,
    stock           INTEGER         NOT NULL,
    price           FLOAT,
    manufacturer    TEXT,
    label           TEXT,

);

CREATE TABLE IF NOT EXISTS component_origin
(
    origin_id       uuid DEFAULT uuidv7()   PRIMARY KEY,
    component_id    uuid         NOT NULL,

    origin          TEXT            NOT NULL,
    part_number     TEXT,
    
    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_file
(
    file_id         uuid DEFAULT uuidv7()   PRIMARY KEY,
    component_id    uuid,

    name            TEXT,
    mime            TEXT,

    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_image
(
    component_id    uuid PRIMARY KEY,

    full            BYTEA,
    thumb           BYTEA,

    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

-- Types

CREATE TABLE IF NOT EXISTS type
(
    type_id     uuid DEFAULT uuidv7()   PRIMARY KEY,

    name        TEXT            NOT NULL,
    fields      jsonb           NOT NULL,
    schema      jsonb           NOT NULL,
);

CREATE TABLE IF NOT EXISTS type_instance
(
    type_instance_id    uuid DEFAULT uuidv7()   PRIMARY KEY,
    type_id             uuid    NOT NULL,

    path                ltree   NOT NULL,

    FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS component_type
(

    component_id        uuid    NOT NULL,
    type_instance_id    uuid    NOT NULL,

    attributes          jsonb   NOT NULL,

    PRIMARY KEY(component_id, type_id),
    
    FOREIGN KEY(component_id)       REFERENCES component(component_id)          ON DELETE CASCADE,
    FOREIGN KEY(type_instance_id)   REFERENCES type_instance(type_instance_id)  ON DELETE CASCADE
);

CREATE INDEX attribute_index ON component_type USING GIN (attributes);
CREATE INDEX type_instance_index ON type_instance USING GIST (path)






-- CREATE TABLE IF NOT EXISTS prompt
-- (

--     type_id     BIGINT         NOT NULL,

--     attribute   TEXT            NOT NULL,
--     value       TEXT            NOT NULL,
--     count       INTEGER         NOT NULL,

--     PRIMARY KEY(type_id, attribute, value),
--     FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
    
-- )