alter table maps
  drop constraint maps_game_title_id_fkey,
  add constraint maps_game_title_id_fkey foreign key (game_title_id) references game_titles (game_title_id);

