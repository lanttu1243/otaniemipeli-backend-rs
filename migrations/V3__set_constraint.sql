ALTER TABLE games
    DROP CONSTRAINT IF EXISTS games_board_fkey,
    ADD  CONSTRAINT games_board_fkey
        FOREIGN KEY (board)
            REFERENCES boards(board_id)
            ON DELETE CASCADE;

ALTER TABLE teams
    DROP CONSTRAINT IF EXISTS teams_game_fkey,
    ADD  CONSTRAINT teams_game_fkey
        FOREIGN KEY (game)
            REFERENCES games(game_id)
            ON DELETE CASCADE;

ALTER TABLE places
    DROP CONSTRAINT IF EXISTS places_drink_fkey,
    ADD  CONSTRAINT places_drink_fkey
        FOREIGN KEY (drink)
            REFERENCES drinks(drink_id)
            ON DELETE CASCADE;

ALTER TABLE turns
    DROP CONSTRAINT IF EXISTS turns_team_fkey,
    ADD  CONSTRAINT turns_team_fkey
        FOREIGN KEY (team)
            REFERENCES teams(team_id)
            ON DELETE CASCADE,
    DROP CONSTRAINT IF EXISTS turns_game_fkey,
    ADD  CONSTRAINT turns_game_fkey
        FOREIGN KEY (game)
            REFERENCES games(game_id)
            ON DELETE CASCADE;

ALTER TABLE drink_ingredients
    DROP CONSTRAINT IF EXISTS drink_ingredients_drink_id_fkey,
    ADD  CONSTRAINT drink_ingredients_drink_id_fkey
        FOREIGN KEY (drink_id)
            REFERENCES drinks(drink_id)
            ON DELETE CASCADE,
    DROP CONSTRAINT IF EXISTS drink_ingredients_ingredient_id_fkey,
    ADD  CONSTRAINT drink_ingredients_ingredient_id_fkey
        FOREIGN KEY (ingredient_id)
            REFERENCES ingredients(ingredient_id)
            ON DELETE CASCADE;

ALTER TABLE turn_drinks
    DROP CONSTRAINT IF EXISTS turn_drinks_drink_id_fkey,
    ADD  CONSTRAINT turn_drinks_drink_id_fkey
        FOREIGN KEY (drink_id)
            REFERENCES drinks(drink_id)
            ON DELETE CASCADE,
    DROP CONSTRAINT IF EXISTS turn_drinks_turn_id_fkey,
    ADD  CONSTRAINT turn_drinks_turn_id_fkey
        FOREIGN KEY (turn_id)
            REFERENCES turns(turn_id)
            ON DELETE CASCADE;

ALTER TABLE board_places
    DROP CONSTRAINT IF EXISTS board_places_board_id_fkey,
    ADD  CONSTRAINT board_places_board_id_fkey
        FOREIGN KEY (board_id)
            REFERENCES boards(board_id)
            ON DELETE CASCADE,
    DROP CONSTRAINT IF EXISTS board_places_place_id_fkey,
    ADD  CONSTRAINT board_places_place_id_fkey
        FOREIGN KEY (place_id)
            REFERENCES places(place_id)
            ON DELETE CASCADE;
