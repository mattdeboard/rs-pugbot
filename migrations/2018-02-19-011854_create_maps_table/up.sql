create table maps (
  game_title_id integer references game_titles not null,
  map_name varchar not null,
  primary key (game_title_id, map_name)
);

