CREATE TYPE place_types AS ENUM ('normal', 'food', 'sauna', 'special', 'guild');
CREATE TYPE user_types AS ENUM ('admin', 'ie', 'referee');

CREATE TABLE boards (
    board_id        SERIAL PRIMARY KEY,
    name            TEXT
);
CREATE TABLE games (
    game_id         SERIAL PRIMARY KEY,
    date            DATE DEFAULT CURRENT_TIMESTAMP,
    name            TEXT DEFAULT '',
    finished        BOOLEAN DEFAULT false,
    board_id        INTEGER REFERENCES boards(board_id)
);
CREATE TABLE teams (
    team_id         SERIAL PRIMARY KEY,
    game_id         INTEGER REFERENCES games(game_id),
    team_name            TEXT,
    team_hash       TEXT
);
CREATE TABLE users (
    uid             SERIAL PRIMARY KEY,
    username        TEXT,
    password        TEXT,
    user_type            user_types
);
CREATE TABLE drinks (
    drink_id        SERIAL PRIMARY KEY,
    name            TEXT
);
CREATE TABLE places (
    place_id        SERIAL PRIMARY KEY,
    place_name      TEXT,
    rule            TEXT DEFAULT '',
    place_type      place_types NOT NULL
);
CREATE TABLE ingredients (
    ingredient_id   SERIAL PRIMARY KEY,
    name            TEXT,
    abv             FLOAT,
    carbonated      BOOLEAN
);
CREATE TABLE turns (
    turn_id         SERIAL PRIMARY KEY,
    start_time      DATE DEFAULT CURRENT_TIMESTAMP,
    team_id         INTEGER REFERENCES teams(team_id),
    game_id         INTEGER REFERENCES games(game_id),
    dice1           INTEGER,
    dice2           INTEGER
);
CREATE TABLE penalties (
    penalty_id      SERIAL PRIMARY KEY,
    team_id         INTEGER REFERENCES teams(team_id),
    turn_id         INTEGER REFERENCES turns(turn_id),
    drink_id        INTEGER REFERENCES drinks(drink_id)
);

-- Relation-tables
CREATE TABLE drink_ingredients (
    drink_id        INTEGER REFERENCES drinks(drink_id),
    ingredient_id   INTEGER REFERENCES ingredients(ingredient_id),
    quantity        FLOAT,
    PRIMARY KEY (drink_id, ingredient_id)
);
CREATE TABLE turn_drinks (
    drink_id        INTEGER REFERENCES drinks(drink_id),
    turn_id         INTEGER REFERENCES turns(turn_id),
    n               INTEGER DEFAULT 1,
    PRIMARY KEY (drink_id, turn_id)
);
CREATE TABLE board_places (
    board_id        INTEGER REFERENCES boards(board_id),
    place_number    INTEGER UNIQUE,
    place_id        INTEGER REFERENCES places(place_id),
    start           BOOLEAN default FALSE,
    "end"           BOOLEAN default FALSE,
    x               FLOAT default 0.0,
    y               FLOAT default 0.0,
    PRIMARY KEY (board_id, place_number)
);
CREATE TABLE place_drinks (
    drink_id        INTEGER REFERENCES drinks(drink_id),
    place_number    INTEGER REFERENCES board_places(place_number),
    board_id        INTEGER REFERENCES boards(board_id),
    refill          BOOLEAN default FALSE,
    optional        BOOLEAN default FALSE,
    n               INTEGER default 1,
    n_update        TEXT default '',
    PRIMARY KEY (drink_id, place_number, board_id)
);
CREATE TABLE place_connections (
    board_id        INTEGER REFERENCES boards(board_id),
    origin          INTEGER REFERENCES board_places(place_number),
    target          INTEGER REFERENCES board_places(place_number),
    on_land         BOOLEAN default FALSE,
    backwards       BOOLEAN default FALSE,
    dashed          BOOLEAN default FALSE,
    PRIMARY KEY (board_id, origin, target)
);
