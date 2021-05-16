alter table game_configs
  drop constraint game_configs_game_mode_id_fkey,
  add constraint game_configs_game_mode_id_fkey foreign key (game_mode_id) references game_modes (game_mode_id) on delete cascade;

