create table if not exists users (
id serial primary key,
username varchar(32) unique not null,
password varchar(255) not null,
email varchar(100) unique not null,
role varchar(50) not null,
created_at timestamp default current_timestamp,
updated_at timestamp default current_timestamp
);