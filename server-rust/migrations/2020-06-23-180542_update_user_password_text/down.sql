-- This file should undo anything in `up.sql`
alter table users alter column password varchar(20) NOT NULL;