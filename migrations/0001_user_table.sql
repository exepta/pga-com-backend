create EXTENSION if not exists "uuid-ossp";

create table if not exists users (
id TEXT primary key default uuid_generate_v4()::TEXT,
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