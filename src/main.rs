use axum::{Extension, Router};
use axum::routing::post;
use firebase_auth_sdk::FireAuth;
use crate::controller::{sign_in, sign_up};

mod controller;
mod model;


#[tokio::main]
async fn main() {
    // Firebase Auth
    // 1) SignUp
    // 2) SignIN

    let auth_service = FireAuth::new(String::from("YOUR-FIREBASE-KEY"));

    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/signup", post(sign_up))
        .layer(Extension(auth_service));

    let listener =
    tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Listening...!");

    axum::serve(listener, app)
        .await
        .unwrap();

}
