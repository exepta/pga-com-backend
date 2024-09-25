create table if not exists tokens (
id serial primary key,
user_id TEXT REFERENCES users(id),
access_token text not null,
refresh_token text not null
);