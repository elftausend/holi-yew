drop table if exists Entries;
create table Entries (
    user_id integer primary key,
    uploaded int[],
    fav int[],
);