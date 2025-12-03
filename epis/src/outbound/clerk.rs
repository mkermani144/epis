//! Clerk auth and user management, implementing [UserManagement]

use std::sync::Arc;

use clerk_rs::validators::{authorizer::validate_jwt, jwks::MemoryCacheJwksProvider};
use derive_more::Constructor;

use crate::domain::{
  models::{AuthStatus, CefrLevel, EpisError, User, UserId},
  ports::UserManagement,
};

/// A wrapper around [clerk_rs::clerk::Clerk] that implements [UserManagement]
#[derive(Clone, Constructor)]
pub struct Clerk(clerk_rs::clerk::Clerk);

impl UserManagement for Clerk {
  async fn authenticate_jwt(&self, jwt: &str) -> Result<AuthStatus, EpisError> {
    match validate_jwt(jwt, Arc::new(MemoryCacheJwksProvider::new(self.0.clone()))).await {
      Ok(clerk_jwt) => Ok(AuthStatus::Authenticated(User::new(
        clerk_jwt.sub,
        clerk_jwt
          .other
          .get("credit")
          .unwrap_or_default()
          .as_number()
          .ok_or(EpisError::Unknown)?
          .as_i64()
          .unwrap_or(0) as i32,
      ))),
      Err(_) => Ok(AuthStatus::Unauthenticated),
    }
  }
  async fn spend_credit(&self, user_id: &UserId) -> Result<(), EpisError> {
    todo!()
  }

  async fn get_cefr_level(&self, user_id: &UserId) -> Result<Option<CefrLevel>, EpisError> {
    todo!()
  }
}
