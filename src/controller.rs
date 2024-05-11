use std::future::Future;
use axum::{Extension, Json};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use firebase_auth_sdk::{Error, FireAuth};
use crate::model::{CredsRequest, Response};

pub async fn sign_in(service: Extension<FireAuth>, Json(creds_request): Json<CredsRequest>)
 -> impl IntoResponse {
    match service.sign_in_email(creds_request.email.as_str(), creds_request.password.as_str(), true).await {
        Ok(response) => {
            (
                StatusCode::OK,
                [(header::AUTHORIZATION, String::from(response.id_token))],
                "Successfully LoggedIn"
           )
        }
        Err(ex) => {
            eprintln!("{:?}", ex);
            (
                StatusCode::UNAUTHORIZED,
                [(header::AUTHORIZATION, String::new())],
                "Invalid Credentials"
            )
        }
    }
}


pub async fn sign_up(service: Extension<FireAuth>, Json(creds_request): Json<CredsRequest>)
 -> Result<Json<Response>, StatusCode> {
    match service.sign_up_email(creds_request.email.as_str(), creds_request.password.as_str(), false).await {
        Ok(_) => {
            let msg = Response {message: String::from("Successfully Registered, Please login...!") };
            Ok(Json(msg))
        }
        Err(ex) => {
            eprintln!("{:?}", ex);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
