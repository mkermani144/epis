//! Clerk auth and user management, implementing [UserManagement]

use std::{str::FromStr, sync::Arc};

use clerk_rs::{
  apis::users_api::User as ClerkUserApi,
  validators::{authorizer::validate_jwt, jwks::MemoryCacheJwksProvider},
};
use derive_getters::Getters;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::domain::{
  models::{AuthStatus, CefrLevel, ChatMateLanguage, EpisError, User, UserId},
  ports::UserManagement,
};

/// Combination of a language and user's CEFR level
#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct UserCefrLevel {
  cefr_level: String,
  language: String,
}

/// Clerk metadata of each user
#[derive(Debug, Clone, Deserialize, Serialize, Getters)]
#[allow(clippy::missing_docs_in_private_items)]
pub struct ClerkUserMetadata {
  credit: i32,
  cefr_level: Vec<UserCefrLevel>,
}

/// A wrapper around [clerk_rs::clerk::Clerk] that implements [UserManagement]
#[derive(Clone, Constructor)]
pub struct Clerk(clerk_rs::clerk::Clerk);

impl Clerk {
  /// Get user's Clerk metadata
  async fn get_user_metadata(&self, user_id: &UserId) -> anyhow::Result<ClerkUserMetadata> {
    let metadata_value = ClerkUserApi::get_user(&self.0, user_id)
      .await?
      .public_metadata
      .unwrap_or_default();
    let metadata: ClerkUserMetadata = serde_json::from_value(metadata_value)?;

    Ok(metadata)
  }
}

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
    let user_metadata = self
      .get_user_metadata(user_id)
      .await
      .map_err(|_| EpisError::Unknown)?;

    ClerkUserApi::update_user_metadata(
      &self.0,
      user_id,
      Some(clerk_rs::models::UpdateUserMetadataRequest {
        public_metadata: Some(json!({
            "credit": user_metadata.credit.saturating_sub(1),
            "cefr_level": user_metadata.cefr_level,
        })),
        private_metadata: None,
        unsafe_metadata: None,
      }),
    )
    .await
    .map_err(|_| EpisError::Unknown)?;

    Ok(())
  }

  async fn get_cefr_level(
    &self,
    user_id: &UserId,
    language: &ChatMateLanguage,
  ) -> Result<Option<CefrLevel>, EpisError> {
    let user_metadata = self
      .get_user_metadata(user_id)
      .await
      .map_err(|_| EpisError::Unknown)?;

    let cefr_level = user_metadata.cefr_level.into_iter().find_map(|cefr_item| {
      if cefr_item.language == language.to_string() {
        return Some(cefr_item.cefr_level);
      }
      None
    });

    Ok(match cefr_level {
      Some(cefr_level) => Some(CefrLevel::from_str(&cefr_level).map_err(|_| EpisError::Unknown)?),
      None => None,
    })
  }
}
