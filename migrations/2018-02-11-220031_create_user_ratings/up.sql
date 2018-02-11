create table user_ratings (
  id serial primary key,
  user_id integer references users,
  rating numeric(2) not null
);
