create table game_configs (
  game_config_id serial primary key,
  game_title_id integer references game_titles not null,
  team_count integer not null,
  team_size integer not null
); 