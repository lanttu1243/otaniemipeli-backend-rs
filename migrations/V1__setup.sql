CREATE TABLE IF NOT EXISTS boards (
    board_id        SERIAL PRIMARY KEY,
    name            TEXT
);
CREATE TABLE IF NOT EXISTS games (
    game_id         SERIAL PRIMARY KEY,
    date            DATE DEFAULT CURRENT_TIMESTAMP,
    name            TEXT DEFAULT '',
    finished        BOOLEAN DEFAULT false,
    board_id        INTEGER REFERENCES boards(board_id)
);
CREATE TABLE IF NOT EXISTS teams (
    team_id         SERIAL PRIMARY KEY,
    game_id            INTEGER REFERENCES games(game_id),
    name            TEXT
);
CREATE TABLE IF NOT EXISTS drinks (
    drink_id        SERIAL PRIMARY KEY,
    name            TEXT
);
CREATE TABLE IF NOT EXISTS places (
    place_id        SERIAL PRIMARY KEY,
    place_name      TEXT,
    rule            TEXT DEFAULT ''
);
CREATE TABLE IF NOT EXISTS ingredients (
    ingredient_id   SERIAL PRIMARY KEY,
    name            TEXT,
    abv             FLOAT,
    carbonated      BOOLEAN
);
CREATE TABLE IF NOT EXISTS turns (
    turn_id         SERIAL PRIMARY KEY,
    start_time      DATE DEFAULT CURRENT_TIMESTAMP,
    team_id         INTEGER REFERENCES teams(team_id),
    game_id         INTEGER REFERENCES games(game_id),
    dice1           INTEGER,
    dice2           INTEGER
);
CREATE TABLE IF NOT EXISTS penalties (
    penalty_id      SERIAL PRIMARY KEY,
    team_id         INTEGER REFERENCES teams(team_id),
    turn_id         INTEGER REFERENCES turns(turn_id),
    drink_id        INTEGER REFERENCES drinks(drink_id)
);

-- Relation-tables
CREATE TABLE IF NOT EXISTS drink_ingredients (
    drink_id        INTEGER REFERENCES drinks(drink_id),
    ingredient_id   INTEGER REFERENCES ingredients(ingredient_id),
    quantity        FLOAT,
    PRIMARY KEY (drink_id, ingredient_id)
);
CREATE TABLE IF NOT EXISTS turn_drinks (
    drink_id        INTEGER REFERENCES drinks(drink_id),
    turn_id         INTEGER REFERENCES turns(turn_id),
    n               INTEGER DEFAULT 1,
    PRIMARY KEY (drink_id, turn_id)
);
CREATE TABLE IF NOT EXISTS board_places (
    board_id        INTEGER REFERENCES boards(board_id),
    place_number    INTEGER UNIQUE,
    place_id        INTEGER REFERENCES places(place_id),
    start           BOOLEAN default FALSE,
    "end"           BOOLEAN default FALSE,
    PRIMARY KEY (board_id, place_number)
);
CREATE TABLE IF NOT EXISTS place_drinks (
    drink_id        INTEGER REFERENCES drinks(drink_id),
    place_id        INTEGER REFERENCES places(place_id),
    refill          BOOLEAN default FALSE,
    optional        BOOLEAN default FALSE,
    "default"       BOOLEAN default TRUE
);
