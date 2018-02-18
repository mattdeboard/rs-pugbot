create table game_modes (
  game_mode_id serial primary key,
  game_title_id integer references game_titles not null,
  mode_name varchar not null,
  unique(game_title_id, mode_name)
);
