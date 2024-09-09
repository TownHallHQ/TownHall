use std::str::FromStr;

use anyhow::Result;
use leptos::{RwSignal, SignalGetUntracked, SignalSet};

use townhall_client::Client;
use townhall_types::user::{Email, User, Username};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum UserSession {
    #[default]
    Unknown,
    Authenticated(User),
    Unauthenticated,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionContext {
    pub user: RwSignal<UserSession>,
}

impl SessionContext {
    pub async fn whoami(&self) -> Result<bool> {
        if matches!(self.user.get_untracked(), UserSession::Unauthenticated)
            || matches!(self.user.get_untracked(), UserSession::Unknown)
        {
            let client = Client::new("http://127.0.0.1:8080")?;
            let res = client.auth.me().await?;

            if let Some(ref user) = res.user {
                let user = user.to_owned();
                let email = Email::from_str(&user.email)?;
                let username = Username::from_str(&user.username)?;

                self.user.set(UserSession::Authenticated(User {
                    email,
                    username,
                    id: user.id,
                    name: user.name.clone(),
                    surname: user.surname.clone(),
                    avatar_id: None,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    deleted_at: None,
                }));

                return Ok(true);
            }
        }

        self.user.set(UserSession::Unauthenticated);
        Ok(false)
    }

    pub async fn login(&self, email: String, password: String) -> Result<bool> {
        let client = Client::new("http://127.0.0.1:8080")?;
        client.auth.login(email, password).await?;
        self.whoami().await
    }
}
