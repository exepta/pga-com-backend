create table if not exists users (
id serial primary key,
uid varchar(255) unique not null,
username varchar(32) unique not null,
password varchar(255) not null,
email varchar(100) unique not null,
role varchar(50) not null,
birthday varchar(12),
avatar_path varchar(255),
banner_path varchar(255),
configurations text,
created_at timestamp default current_timestamp,
updated_at timestamp default current_timestamp
);