table! {
    game_configs (game_config_id) {
        game_config_id -> Int4,
        team_count -> Int4,
        team_size -> Int4,
        game_mode_id -> Int4,
    }
}

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
    maps (game_title_id, map_name) {
        game_title_id -> Int4,
        map_name -> Varchar,
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
        discord_user_id -> Int4,
    }
}

joinable!(game_configs -> game_modes (game_mode_id));
joinable!(game_modes -> game_titles (game_title_id));
joinable!(maps -> game_titles (game_title_id));
joinable!(user_ratings -> game_modes (game_mode_id));
joinable!(user_ratings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    game_configs,
    game_modes,
    game_titles,
    maps,
    user_ratings,
    users,
);
