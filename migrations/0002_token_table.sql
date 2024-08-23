create table if not exists tokens (
id serial primary key,
uid varchar(255) unique not null,
access_token text not null,
refresh_token text not null
)