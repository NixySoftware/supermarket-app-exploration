use std::ops::Deref;

use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use supermarket_web_database::entities::user;
use supermarket_web_database::sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait};

#[derive(Debug, Clone)]
pub enum Credentials {
    Oidc,
}

#[derive(Debug, Clone)]
pub struct UserWrapper(user::Model);

impl AuthUser for UserWrapper {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.0.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        todo!()
    }
}

impl Deref for UserWrapper {
    type Target = user::Model;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<user::Model> for UserWrapper {
    fn from(value: user::Model) -> Self {
        UserWrapper(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Database(#[from] DbErr),
}

#[derive(Debug, Clone)]
pub struct Backend {
    database: DatabaseConnection,
}

impl Backend {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type Credentials = Credentials;
    type Error = BackendError;
    type User = UserWrapper;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        todo!()
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(user::Entity::find_by_id(*user_id)
            .one(&self.database)
            .await?
            .map(UserWrapper::from))
    }
}
