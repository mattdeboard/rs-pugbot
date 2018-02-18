alter table user_ratings add column game_mode_id integer references game_modes not null;
