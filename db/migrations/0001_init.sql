CREATE TABLE IF NOT EXISTS components
(
    id INTEGER PRIMARY KEY,
    name        TEXT            NOT NULL,
    stock       INTEGER         NOT NULL,
    price       FLOAT,
    origin      TEXT,
    label       TEXT,
    image       BOOLEAN         NOT NULL,
    datasheet   BOOLEAN         NOT NULL,
    attribute_id INTEGER         NOT NULL,
    attributes  TEXT            NOT NULL
);

CREATE TABLE IF NOT EXISTS types
(
    id INTEGER PRIMARY KEY,
    name        TEXT            NOT NULL,
    attributes  TEXT            NOT NULL,
    schema      TEXT            NOT NULL,
    prompts     TEXT            NOT NULL
);