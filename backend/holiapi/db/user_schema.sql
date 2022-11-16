create table users (
    user_id varchar(13) primary key not null,
    username varchar(120),
    entry_info json,
    flag_count integer default 0,
    banned integer default 0,
    class varchar(7)
);