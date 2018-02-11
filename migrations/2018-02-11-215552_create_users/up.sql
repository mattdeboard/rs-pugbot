create table users (
  user_id serial primary key,
  bot bool not null default false,
  discriminator varchar not null,
  name varchar not null
);
