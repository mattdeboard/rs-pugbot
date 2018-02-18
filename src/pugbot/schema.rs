table! {
    game_modes (game_mode_id) {
        game_mode_id -> Int4,
        game_title_id -> Int4,
        mode_name -> Varchar,
        team_size -> Int4,
    }
}

table! {
    game_titles (game_title_id) {
        game_title_id -> Int4,
        game_name -> Varchar,
    }
}

table! {
    user_ratings (id) {
        id -> Int4,
        user_id -> Int4,
        rating -> Numeric,
        deviation -> Numeric,
        volatility -> Numeric,
        game_mode_id -> Int4,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        bot -> Bool,
        discriminator -> Int4,
        name -> Varchar,
    }
}

joinable!(user_ratings -> users (user_id));
joinable!(game_modes -> game_titles (game_title_id));
joinable!(user_ratings -> game_modes (game_mode_id));
