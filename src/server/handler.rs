use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form, Json,
};

use crate::server::login::{AuthSession};

use super::login::Credentials;

pub async fn login(
    mut auth_session: AuthSession,
    Json(creds): Json<Credentials>,
) -> Result<impl IntoResponse, StatusCode> {

    println!("username: {0}, password: {1}", creds.username,creds.password);

    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if auth_session.login(&user).await.is_err() {
        println!("THINGS DID NOT GO WELL");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    println!("redireccting....");
    Ok(Redirect::to("/test").into_response())
}