create table game_titles (
  game_title_id serial primary key,
  game_name varchar not null,
  unique(game_name)
);
