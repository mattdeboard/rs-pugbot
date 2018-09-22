with ow_id as ( select game_title_id from game_titles where game_name = 'Overwatch' )
insert into maps (game_title_id, map_name) values
((select game_title_id from ow_id), 'Blizzard World'),
((select game_title_id from ow_id), 'Hanamura'),
((select game_title_id from ow_id), 'Horizon Lunar Colony'),
((select game_title_id from ow_id), 'Junkertown'),
((select game_title_id from ow_id), 'Ilios'),
((select game_title_id from ow_id), 'Numbani'),
((select game_title_id from ow_id), 'Nepal'),
((select game_title_id from ow_id), 'Temple of Anubis'),
((select game_title_id from ow_id), 'Kings Row'),
((select game_title_id from ow_id), 'Volskaya Industries'),
((select game_title_id from ow_id), 'Dorado'),
((select game_title_id from ow_id), 'Route 66'),
((select game_title_id from ow_id), 'Watchpoint: Gibraltar'),
((select game_title_id from ow_id), 'Eichenwalde'),
((select game_title_id from ow_id), 'Hollywood'),
((select game_title_id from ow_id), 'Lijiang Tower'),
((select game_title_id from ow_id), 'Oasis');
