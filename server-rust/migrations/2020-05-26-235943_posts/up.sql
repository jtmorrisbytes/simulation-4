-- Your SQL goes here
CREATE TABLE posts (
 id serial primary key,
 title varchar(45),
 img text,
 content text,
 author_id integer references users(id)
);