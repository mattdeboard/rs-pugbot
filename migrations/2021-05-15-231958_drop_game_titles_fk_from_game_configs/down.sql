alter table game_configs
  add column game_title_id integer references game_titles;

