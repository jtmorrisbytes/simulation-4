-- Your SQL goes here
create table users(
  id SERIAL PRIMARY KEY,
  username varchar(20) UNIQUE NOT NULL,
  password varchar(20) NOT NULL,
  profile_pic text
);

insert into users (username, password, profile_pic)
  values('jtmorrisbytes','jordan','https://randomuser.me/api/portraits/men/11.jpg');