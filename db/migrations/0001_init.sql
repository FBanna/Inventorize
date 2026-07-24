-- Classes

CREATE TABLE IF NOT EXISTS class
(
    class_id     uuid DEFAULT uuidv7()   PRIMARY KEY,

    name        TEXT            NOT NULL,
    fields      jsonb           NOT NULL,
    schema      jsonb           NOT NULL
);

CREATE TABLE IF NOT EXISTS class_instance
(
    class_instance_id   uuid DEFAULT uuidv7()   PRIMARY KEY,
    class_id            uuid    NOT NULL,

    parent              uuid, -- can be null

    FOREIGN KEY(class_id)        REFERENCES class(class_id)  ON DELETE CASCADE,
    FOREIGN KEY(parent)       REFERENCES class_instance(class_instance_id) ON DELETE CASCADE
);


-- Origin
CREATE TABLE IF NOT EXISTS origin
(
    origin_id       uuid DEFAULT uuidv7()   PRIMARY KEY,

    name            TEXT            NOT NULL,
    url             TEXT            NOT NULL
);

-- Components
CREATE TABLE IF NOT EXISTS component
(
    component_id        uuid DEFAULT uuidv7()   PRIMARY KEY,
    class_instance_id   uuid        NOT NULL,

    name            TEXT            NOT NULL,
    stock           INTEGER         NOT NULL,
    manufacturer    TEXT,
    label           TEXT,

    FOREIGN KEY(class_instance_id)  REFERENCES class_instance(class_instance_id) ON DELETE CASCADE

);

CREATE TABLE IF NOT EXISTS component_origin
(
    origin_id       uuid         NOT NULL,
    component_id    uuid         NOT NULL,

    part_number     TEXT,
    price           NUMERIC,
    
    PRIMARY KEY(origin_id, component_id),
    FOREIGN KEY(origin_id)          REFERENCES origin(origin_id)        ON DELETE CASCADE,
    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_file
(
    file_id         uuid DEFAULT uuidv7()   PRIMARY KEY,
    component_id    uuid    NOT NULL,

    name            TEXT    NOT NULL,
    mime            TEXT    NOT NULL,

    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS component_image
(
    component_id    uuid PRIMARY KEY,
    full_img        BYTEA   NOT NULL,
    thumb_img       BYTEA   NOT NULL,

    FOREIGN KEY(component_id)       REFERENCES component(component_id)  ON DELETE CASCADE
);


-- Component <---> Classes

CREATE TABLE IF NOT EXISTS component_class
(

    component_id        uuid    NOT NULL,
    class_instance_id   uuid    NOT NULL,

    attributes          jsonb   NOT NULL,

    PRIMARY KEY(component_id, class_instance_id),
    
    FOREIGN KEY(component_id)       REFERENCES component(component_id)          ON DELETE CASCADE,
    FOREIGN KEY(class_instance_id)   REFERENCES class_instance(class_instance_id)  ON DELETE CASCADE
);

-- Indexes

CREATE INDEX attribute_index ON component_class USING GIN (attributes);