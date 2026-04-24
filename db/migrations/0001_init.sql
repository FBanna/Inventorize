CREATE TABLE IF NOT EXISTS component
(
    component_id BIGSERIAL PRIMARY KEY,
    name            TEXT            NOT NULL,
    stock           INTEGER         NOT NULL,
    price           FLOAT,
    manufacturer    TEXT,
    label           TEXT,

);

CREATE TABLE IF NOT EXISTS origin
(
    origin_id BIGSERIAL PRIMARY KEY,
    origin          TEXT            NOT NULL,
    part_number     TEXT,
    component_id    BIGINT         NOT NULL,
    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_file
(
    file_id UUID PRIMARY KEY,
    component_id    BIGINT,
    name            TEXT,
    mime            TEXT,
    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_image
(
    component_id    BIGINT PRIMARY KEY,
    full            BYTEA,
    thumb           BYTEA,
    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS type
(
    type_id BIGSERIAL PRIMARY KEY ,
    name        TEXT            NOT NULL,
    inherits    BIGINT,

    FOREIGN KEY(inherits)        REFERENCES type(type_id)  ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS component_type
(

    component_id    BIGINT,
    type_id         BIGINT,

    attributes      jsonb        NOT NULL,

    PRIMARY KEY(component_id, type_id),
    
    FOREIGN KEY(component_id)   REFERENCES component(component_id)  ON DELETE CASCADE,
    FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
);

CREATE INDEX attribute_index ON component_type USING GIN (attributes);




CREATE TABLE IF NOT EXISTS type_attribute
(
    
    type_id     BIGINT          NOT NULL,
    
    fields      jsonb           NOT NULL,
    schema      jsonb           NOT NULL,
    -- prompts     jsonb            NOT NULL,

    PRIMARY KEY(type_id),
    FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
);

-- CREATE TABLE IF NOT EXISTS prompt
-- (

--     type_id     BIGINT         NOT NULL,

--     attribute   TEXT            NOT NULL,
--     value       TEXT            NOT NULL,
--     count       INTEGER         NOT NULL,

--     PRIMARY KEY(type_id, attribute, value),
--     FOREIGN KEY(type_id)        REFERENCES type(type_id)  ON DELETE CASCADE
    
-- )