use diesel::prelude::*;

#[derive(Queryable, PartialEq, Eq, Debug)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub entry_info: String,
    pub flag_count: i32,
    pub banned: i32,
    pub class: String
}