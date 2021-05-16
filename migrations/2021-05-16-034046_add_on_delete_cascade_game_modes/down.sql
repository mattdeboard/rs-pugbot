alter table game_modes
  drop constraint game_modes_game_title_id_fkey,
  add constraint game_modes_game_title_id_fkey foreign key (game_title_id) references game_titles (game_title_id);

