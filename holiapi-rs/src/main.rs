mod jwt;
mod db;

use axum::{
    routing::{get, post},
    Json, Router,
};
use db::{establish_connection, User};
use diesel::{QueryDsl, RunQueryDsl};
use jwt::{AuthError, Claims};
use once_cell::sync::Lazy;
use std::net::SocketAddr;

use crate::jwt::authorize;

use db::schema::users::dsl::*;

const TOKEN_URL: &'static str = "https://auth.htl-hl.ac.at/token.php";
const CLIENT_ID: &'static str = "holi.htl-hl.ac.at";

static CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string("../backend/holiapi/client_secret").unwrap()
});

//const CLIENT_SECRET: &'static str = file_contents("client_secret");
const GRANT_TYPE: &'static str = "authorization_code";
const REDIRECT_URI: &'static str = "https://holi.htl-hl.ac.at/authenticated";

const USER_INFO_URL: &'static str = "https://auth.htl-hl.ac.at/getUserInformation.php?access_token=";


#[tokio::main]
async fn main() {
    let connection = &mut establish_connection();
    let a = users.load::<User>(connection).expect("msg");
    
    let app = Router::new()
        .route("/protected", get(protected))
        .route("/auth", post(authorize));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    // Send the protected data to the user
    Ok(format!(
        "Welcome to the protected area :)\nYour data:\n{}",
        claims
    ))
}