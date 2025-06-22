CREATE TABLE IF NOT EXISTS boards (
    board_id        SERIAL PRIMARY KEY,
    name            TEXT
);
CREATE TABLE IF NOT EXISTS games (
    game_id         SERIAL PRIMARY KEY,
    date            DATE DEFAULT CURRENT_TIMESTAMP,
    name            TEXT DEFAULT '',
    board           INTEGER REFERENCES boards(board_id)
);
CREATE TABLE IF NOT EXISTS teams (
    team_id         SERIAL PRIMARY KEY,
    game            INTEGER REFERENCES games(game_id),
    name            TEXT
);
CREATE TABLE IF NOT EXISTS drinks (
    drink_id        SERIAL PRIMARY KEY,
    name            TEXT
);
CREATE TABLE IF NOT EXISTS places (
    place_id        SERIAL PRIMARY KEY,
    place_name      TEXT,
    refill          BOOLEAN,
    drink           INTEGER REFERENCES drinks(drink_id)
);
CREATE TABLE IF NOT EXISTS ingredients (
    ingredient_id   SERIAL PRIMARY KEY,
    name            TEXT,
    abv             FLOAT,
    carbonated      BOOLEAN
);
CREATE TABLE IF NOT EXISTS turns (
    turn_id         SERIAL PRIMARY KEY,
    team            INTEGER REFERENCES teams(team_id),
    game            INTEGER REFERENCES games(game_id),
    dice1           INTEGER,
    dice2           INTEGER
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
    place_number    INTEGER,
    place_id        INTEGER REFERENCES places(place_id),
    PRIMARY KEY (board_id, place_number)
);

