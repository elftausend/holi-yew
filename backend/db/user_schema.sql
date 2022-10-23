drop table if exists Users;
create table Users (
    user_id integer primary key,
    uploaded int[],
    fav int[],
);