insert into game_titles (game_name) values
  ('Overwatch'),
  ('Rocket League')
;

with ow_id as (
  select game_title_id as id
    from game_titles
   where game_name = 'Overwatch'
)
insert into game_modes (game_title_id, mode_name, team_size) values
  ((select id from ow_id), 'Standard', 6),
  ((select id from ow_id), 'Capture The Flag', 6),
  ((select id from ow_id), '1v1', 1),
  ((select id from ow_id), '5v5', 5);

with rl_id as (
  select game_title_id as id
    from game_titles
   where game_name = 'Rocket League'
)
insert into game_modes (game_title_id, mode_name, team_size) values
  ((select id from ow_id), 'Standard 3v3', 3),
  ((select id from ow_id), '2v2', 2)
  ((select id from ow_id), '4v4', 4),
  ((select id from ow_id), '1v1', 1);
