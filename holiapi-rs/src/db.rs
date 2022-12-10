pub mod models;
pub mod schema;

pub use models::*;

use diesel::{SqliteConnection, Connection};

pub fn establish_connection() -> SqliteConnection {
    
    let database_url = "../backend/holiapi/db/user_database.db";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}