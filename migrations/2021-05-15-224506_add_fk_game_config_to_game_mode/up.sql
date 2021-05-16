alter table game_configs
  add column game_mode_id integer references game_modes not null;

