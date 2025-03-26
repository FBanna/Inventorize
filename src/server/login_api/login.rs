use std::collections::HashMap;

use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::{generate_hash, verify_password};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        0
    }

    fn session_auth_hash(&self) -> &[u8] {
        &self.password.as_bytes()
    }
}

#[derive(Clone)]
pub struct Backend {
    user: User,
}

impl Backend {
    pub fn new(user: User) -> Self{

        let hash = generate_hash(user.password);
        Self {user: User{username: user.username, password: hash}}
    } 
}

// #[derive(Clone)]
// struct Credentials {
//     
// }

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        if verify_password(creds.password, &self.user.password).is_ok() && creds.username == self.user.username {

            return Ok(Some(self.user.clone()));
        } else {
            return Ok(None);
        }

    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(Some(self.user.clone()))
    }
}

pub type AuthSession = axum_login::AuthSession<Backend>;