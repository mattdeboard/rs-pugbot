alter table user_ratings alter column rating type numeric(6, 2);
alter table user_ratings alter column rating set default 1500.0;
alter table user_ratings add column deviation numeric(5, 2) not null default 350.0;
alter table user_ratings add column volatility numeric(3, 2) not null default 0.3;
